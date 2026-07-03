use crate::cell::CellType;
use crate::chunk::{Chunk, VoxelChunk};
use crate::matrix::MatrixIndex;
use crate::world_data::{FullIndex, VoxelWorld, World, WorldModified};
use crate::{CHUNK_HEIGHT, CHUNK_HEIGHT_LAST, CHUNK_WIDTH, CHUNK_WIDTH_LAST, ChunkIndexType};
use bevy::diagnostic::FrameCount;
use bevy::prelude::{Local, Res, ResMut};
use rand::distr::{Bernoulli, Uniform};
use rand::rngs::SmallRng;
use rand::{RngExt as _, make_rng};
use std::hint::cold_path;
use std::mem;
use std::time::Instant;
const TIME_PER_CHUNK: u128 = 2048;
pub fn simulate_world(
    mut world: ResMut<World>,
    mut voxel_world: ResMut<VoxelWorld>,
    mut modified: ResMut<WorldModified>,
    mut simulate: Local<u8>,
    frame: Res<FrameCount>,
    mut last: Local<FrameCount>,
) {
    if *last == *frame {
        return;
    }
    *last = *frame;
    modified.visual_modified = true;
    world.simulate(&mut voxel_world, *simulate);
    //TODO can parralelize each edge seperately
    world.simulate_edges(&mut voxel_world, *simulate);
    *simulate = simulate.wrapping_add(1);
}
pub struct WorldRand {
    pub rng: SmallRng,
    pub half: Bernoulli,
    pub gas: Uniform<usize>,
}
impl Default for WorldRand {
    fn default() -> Self {
        Self {
            rng: make_rng(),
            half: Bernoulli::new(0.5).unwrap(),
            gas: Uniform::new(0, 6).unwrap(),
        }
    }
}
impl WorldRand {
    pub fn half(&mut self) -> bool {
        self.rng.sample(self.half)
    }
    pub fn gas(&mut self) -> usize {
        self.rng.sample(self.gas)
    }
}
impl World {
    pub fn simulate(&mut self, voxel_world: &mut VoxelWorld, frame: u8) {
        self.par_iter_zip_mut(voxel_world, |iter| {
            let tmr = Instant::now();
            let mut rand = WorldRand::default();
            let mut need_time = false;
            for (_, chunk, voxel_chunk) in iter.iter_mut() {
                if tmr.elapsed().as_micros() < TIME_PER_CHUNK {
                    if !chunk.skip_simulation {
                        chunk.simulate(voxel_chunk, &mut rand, frame);
                        chunk.skip_simulation = true;
                    }
                } else {
                    chunk.skip_simulation = false;
                    need_time = true;
                }
            }
            if !need_time {
                for (_, chunk, _) in iter {
                    chunk.skip_simulation = false;
                }
            }
        });
    }
    pub fn simulate_edges(&mut self, voxel_world: &mut VoxelWorld, frame: u8) {
        let mut rand = WorldRand::default();
        let min_x = self.chunks.min_elem.x;
        let max_x = self.chunks.max_elem.x;
        let min_y = self.chunks.min_elem.y;
        let max_y = self.chunks.max_elem.y;
        let tmr = Instant::now();
        let mut need_time = false;
        for (x, y) in (min_y..=max_y).flat_map(|y| (min_x..=max_x).map(move |x| (x, y))) {
            let chunk_index = MatrixIndex { x, y };
            if let Some(chunk) = &mut self[chunk_index] {
                if tmr.elapsed().as_micros() < TIME_PER_CHUNK {
                    if !chunk.skip_edge_simulation {
                        chunk.skip_edge_simulation = true;
                        self.simulate_chunk_edges(voxel_world, chunk_index, frame, &mut rand);
                        self.simulate_chunk_corners(voxel_world, chunk_index, frame, &mut rand);
                    }
                } else {
                    chunk.skip_edge_simulation = false;
                    need_time = true;
                }
            }
        }
        if !need_time {
            for (_, chunk) in self.iter_mut() {
                chunk.skip_edge_simulation = false;
            }
        }
    }
    pub fn simulate_chunk_corners(
        &mut self,
        voxel_world: &mut VoxelWorld,
        chunk_index: MatrixIndex,
        frame: u8,
        rand: &mut WorldRand,
    ) {
        for (run, x, y) in [
            (
                self[chunk_index - (0, 1)].is_some()
                    && self[chunk_index - (1, 0)].is_some()
                    && self[chunk_index - (1, 1)].is_some(),
                0,
                0,
            ),
            (
                self[chunk_index - (0, 1)].is_some()
                    && self[chunk_index + (1, 0)].is_some()
                    && self[chunk_index - (0, 1) + (1, 0)].is_some(),
                CHUNK_WIDTH_LAST,
                0,
            ),
            (
                self[chunk_index + (0, 1)].is_some()
                    && self[chunk_index - (1, 0)].is_some()
                    && self[chunk_index - (1, 0) + (0, 1)].is_some(),
                0,
                CHUNK_HEIGHT_LAST,
            ),
            (
                self[chunk_index + (0, 1)].is_some()
                    && self[chunk_index + (1, 0)].is_some()
                    && self[chunk_index + (1, 1)].is_some(),
                CHUNK_WIDTH_LAST,
                CHUNK_HEIGHT_LAST,
            ),
        ] {
            if run {
                let index = FullIndex {
                    cell_index: MatrixIndex { x, y },
                    chunk_index,
                };
                self.simulate_cell(voxel_world, index, frame, rand);
            }
        }
    }
    pub fn simulate_chunk_edges(
        &mut self,
        voxel_world: &mut VoxelWorld,
        chunk_index: MatrixIndex,
        frame: u8,
        rand: &mut WorldRand,
    ) {
        for (run, y) in [
            (self[chunk_index - (0, 1)].is_some(), 0),
            (self[chunk_index + (0, 1)].is_some(), CHUNK_HEIGHT_LAST),
        ] {
            if run {
                for x in 1..CHUNK_WIDTH_LAST {
                    let index = FullIndex {
                        cell_index: MatrixIndex { x, y },
                        chunk_index,
                    };
                    self.simulate_cell(voxel_world, index, frame, rand);
                }
            }
        }
        for (run, x) in [
            (self[chunk_index - (1, 0)].is_some(), 0),
            (self[chunk_index + (1, 0)].is_some(), CHUNK_WIDTH_LAST),
        ] {
            if run {
                for y in 1..CHUNK_HEIGHT_LAST {
                    let index = FullIndex {
                        cell_index: MatrixIndex { x, y },
                        chunk_index,
                    };
                    self.simulate_cell(voxel_world, index, frame, rand);
                }
            }
        }
    }
    pub fn simulate_cell(
        &mut self,
        voxel_world: &mut VoxelWorld,
        index: FullIndex,
        frame: u8,
        rand: &mut WorldRand,
    ) {
        let Some(cell) = self.get_mut(index) else {
            return;
        };
        if cell.last_changed == frame {
            return;
        }
        cell.last_changed = frame;
        match cell.cell_type {
            CellType::Liquid => {
                let check = if rand.half() {
                    [
                        index - (0, 1),
                        index - (0, 1) + (1, 0),
                        index - (0, 1) - (1, 0),
                        index + (1, 0),
                        index - (1, 0),
                    ]
                } else {
                    [
                        index - (0, 1),
                        index - (0, 1) - (1, 0),
                        index - (0, 1) + (1, 0),
                        index - (1, 0),
                        index + (1, 0),
                    ]
                };
                self.swap_from_list(voxel_world, index, &check);
            }
            CellType::Granular => {
                let check = if rand.half() {
                    [
                        index - (0, 1),
                        index - (0, 1) + (1, 0),
                        index - (0, 1) - (1, 0),
                    ]
                } else {
                    [
                        index - (0, 1),
                        index - (0, 1) - (1, 0),
                        index - (0, 1) + (1, 0),
                    ]
                };
                self.swap_from_list(voxel_world, index, &check);
            }
            CellType::Gas => {
                let check = [match rand.gas() {
                    0 => index + (0, 1) - (1, 0),
                    1 => index + (0, 1),
                    2 => index + (1, 1),
                    3 => index - (1, 0),
                    4 => index,
                    5 => index + (1, 0),
                    _ => unreachable!(),
                }];
                self.swap_from_list(voxel_world, index, &check);
            }
            _ => {}
        }
    }
    pub fn swap_from_list(
        &mut self,
        voxel_world: &mut VoxelWorld,
        index: FullIndex,
        check: &[FullIndex],
    ) {
        for swap_index in check.iter().copied() {
            if self.try_swap(voxel_world, index, swap_index) {
                return;
            }
        }
    }
    pub fn try_swap(
        &mut self,
        voxel_world: &mut VoxelWorld,
        index: FullIndex,
        swap_index: FullIndex,
    ) -> bool {
        if let Some(cell) = self.get(index)
            && let Some(other) = self.get(swap_index)
            && cell.can_move(other)
        {
            self.swap(voxel_world, index, swap_index);
            true
        } else {
            false
        }
    }
    pub fn swap(&mut self, voxel_world: &mut VoxelWorld, index: FullIndex, swap_index: FullIndex) {
        if let Some(cell) = self.get(index)
            && let Some(other) = self.get(swap_index)
        {
            let is_a = cell.is_collider();
            let is_b = other.is_collider();
            if is_a != is_b {
                voxel_world.add_voxel(swap_index, is_a);
                voxel_world.add_voxel(index, is_b);
            }
        }
        if index.chunk_index == swap_index.chunk_index {
            if let Some(chunk) = &mut self[index.chunk_index] {
                chunk.cells.swap(index.cell_index, swap_index.cell_index);
            } else {
                cold_path();
            }
        } else if let [Some(from), Some(to)] = self
            .chunks
            .matrix
            .get_disjoint_mut([index.chunk_index, swap_index.chunk_index])
        {
            mem::swap(&mut from[index.cell_index], &mut to[swap_index.cell_index]);
        } else {
            cold_path();
        }
    }
}
impl Chunk {
    pub fn simulate(&mut self, voxel: &mut VoxelChunk, rand: &mut WorldRand, frame: u8) {
        for y in (1..=(CHUNK_HEIGHT - 2).strict_cast::<ChunkIndexType>()).rev() {
            for x in 1..=(CHUNK_WIDTH - 2).strict_cast::<ChunkIndexType>() {
                let index = MatrixIndex {
                    x: if y.is_multiple_of(2) {
                        x
                    } else {
                        CHUNK_WIDTH_LAST - x
                    },
                    y,
                };
                self.simulate_cell(voxel, index, rand, frame);
            }
        }
    }
    pub fn simulate_cell(
        &mut self,
        voxel: &mut VoxelChunk,
        index: MatrixIndex,
        rand: &mut WorldRand,
        frame: u8,
    ) {
        if self[index].last_changed == frame {
            return;
        }
        self[index].last_changed = frame;
        match self[index].cell_type {
            CellType::Liquid => {
                let check = if rand.half() {
                    [
                        index - (0, 1),
                        index - (0, 1) + (1, 0),
                        index - (0, 1) - (1, 0),
                        index + (1, 0),
                        index - (1, 0),
                    ]
                } else {
                    [
                        index - (0, 1),
                        index - (0, 1) - (1, 0),
                        index - (0, 1) + (1, 0),
                        index - (1, 0),
                        index + (1, 0),
                    ]
                };
                self.swap_from_list(voxel, index, &check);
            }
            CellType::Granular => {
                let check = if rand.half() {
                    [
                        index - (0, 1),
                        index - (0, 1) + (1, 0),
                        index - (0, 1) - (1, 0),
                    ]
                } else {
                    [
                        index - (0, 1),
                        index - (0, 1) - (1, 0),
                        index - (0, 1) + (1, 0),
                    ]
                };
                self.swap_from_list(voxel, index, &check);
            }
            CellType::Gas => {
                let check = [match rand.gas() {
                    0 => index + (0, 1) - (1, 0),
                    1 => index + (0, 1),
                    2 => index + (1, 1),
                    3 => index - (1, 0),
                    4 => index,
                    5 => index + (1, 0),
                    _ => unreachable!(),
                }];
                self.swap_from_list(voxel, index, &check);
            }
            _ => {}
        }
    }
    pub fn swap_from_list(
        &mut self,
        voxel: &mut VoxelChunk,
        index: MatrixIndex,
        check: &[MatrixIndex],
    ) {
        for swap_index in check.iter().copied() {
            if self.try_swap(voxel, index, swap_index) {
                return;
            }
        }
    }
    pub fn try_swap(
        &mut self,
        voxel: &mut VoxelChunk,
        index: MatrixIndex,
        swap_index: MatrixIndex,
    ) -> bool {
        if self[index].can_move(&self[swap_index]) {
            self.swap(voxel, index, swap_index);
            true
        } else {
            false
        }
    }
    pub fn swap(&mut self, voxel: &mut VoxelChunk, index: MatrixIndex, swap_index: MatrixIndex) {
        let is_a = self[index].is_collider();
        let is_b = self[swap_index].is_collider();
        if is_a != is_b {
            voxel.add_voxel(swap_index, is_a);
            voxel.add_voxel(index, is_b);
        }
        self.cells.swap(index, swap_index);
    }
}

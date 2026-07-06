use crate::cell::CellType;
use crate::matrix::MatrixIndex;
use crate::world_data::{FullIndex, VoxelWorld, World, WorldModified};
use bevy::diagnostic::FrameCount;
use bevy::prelude::{Local, Res, ResMut};
use rand::distr::{Bernoulli, Uniform};
use rand::rngs::SmallRng;
use rand::{RngExt as _, make_rng};
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
        #[cfg(not(feature = "wasm"))]
        let tmr = Instant::now();
        let mut rand = WorldRand::default();
        let mut need_time = false;
        for x in self.min_elem.y..=self.max_elem.y {
            for y in self.min_elem.x..=self.max_elem.x {
                let chunk_index = MatrixIndex { x, y };
                let Some(chunk) = &mut self[chunk_index] else {
                    continue;
                };
                #[cfg(not(feature = "wasm"))]
                if tmr.elapsed().as_micros() < TIME_PER_CHUNK {
                    if !chunk.skip_simulation {
                        chunk.skip_simulation = true;
                        self.simulate_chunk(voxel_world, chunk_index, &mut rand, frame);
                    }
                } else {
                    chunk.skip_simulation = false;
                    need_time = true;
                }
                if cfg!(feature = "wasm") {
                    self.simulate_chunk(voxel_world, chunk_index, &mut rand, frame);
                }
            }
        }
        #[cfg(not(feature = "wasm"))]
        if !need_time {
            for x in self.min_elem.y..=self.max_elem.y {
                for y in self.min_elem.x..=self.max_elem.x {
                    let chunk_index = MatrixIndex { x, y };
                    if let Some(chunk) = &mut self[chunk_index] {
                        chunk.skip_simulation = false;
                    }
                }
            }
        }
    }
    pub fn simulate_chunk(
        &mut self,
        voxel_world: &mut VoxelWorld,
        chunk_index: MatrixIndex,
        rand: &mut WorldRand,
        frame: u8,
    ) {
        for i in 0..=65535 {
            let index = FullIndex {
                chunk_index,
                cell_index: MatrixIndex::from(i),
            };
            self.simulate_cell(voxel_world, index, rand, frame);
        }
    }
    pub fn simulate_cell(
        &mut self,
        voxel_world: &mut VoxelWorld,
        index: FullIndex,
        rand: &mut WorldRand,
        frame: u8,
    ) {
        let Some(cell) = self.get_mut(index) else {
            return;
        };
        if cell.last_changed == frame {
            return;
        }
        cell.last_changed = frame;
        match cell.cell_type() {
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
                unreachable!()
            }
        } else if let [Some(from), Some(to)] = self
            .chunks
            .matrix
            .get_disjoint_mut([index.chunk_index, swap_index.chunk_index])
        {
            mem::swap(&mut from[index.cell_index], &mut to[swap_index.cell_index]);
        } else {
            unreachable!()
        }
    }
}

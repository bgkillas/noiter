use crate::cell::CellType;
use crate::chunk_map::{ChunkMap, ChunkMapModified, FullIndex, VoxelChunkMap};
use crate::matrix::MatrixIndex;
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, ChunkIndexType};
use bevy::diagnostic::FrameCount;
use bevy::prelude::{Local, Res, ResMut};
use rand::distr::{Bernoulli, Uniform};
use rand::rngs::SmallRng;
use rand::{RngExt as _, make_rng};
use std::mem;
use std::time::Instant;
const TIME_ALLOCATED: u128 = 4096;
pub fn simulate_world(
    mut world: ResMut<ChunkMap>,
    mut voxel_world: ResMut<VoxelChunkMap>,
    mut modified: ResMut<ChunkMapModified>,
    frame: Res<FrameCount>,
    mut first_chunk: Local<usize>,
) {
    world.simulate::<TIME_ALLOCATED>(&mut voxel_world, &mut modified, *frame, &mut first_chunk);
}
impl ChunkMap {
    pub fn simulate<const TIME_ALLOCATED: u128>(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
        modified: &mut ChunkMapModified,
        frame: FrameCount,
        first_chunk: &mut usize,
    ) {
        let mut rand: SmallRng = make_rng();
        let bern = Bernoulli::new(0.5).unwrap();
        let uni = Uniform::new(0, 6).unwrap();
        let tmr = Instant::now();
        if self.go_through_chunks::<TIME_ALLOCATED>(
            voxel_world,
            modified,
            frame,
            first_chunk,
            usize::MAX,
            tmr,
            &mut rand,
            bern,
            uni,
        ) {
            return;
        }
        if *first_chunk != 0 {
            let upto = *first_chunk;
            *first_chunk = 0;
            self.go_through_chunks::<TIME_ALLOCATED>(
                voxel_world,
                modified,
                frame,
                first_chunk,
                upto,
                tmr,
                &mut rand,
                bern,
                uni,
            );
        }
    }
    fn go_through_chunks<const TIME_ALLOCATED: u128>(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
        modified: &mut ChunkMapModified,
        frame: FrameCount,
        first_chunk: &mut usize,
        upto: usize,
        tmr: Instant,
        rand: &mut SmallRng,
        bern: Bernoulli,
        uni: Uniform<usize>,
    ) -> bool {
        let min_x = self.chunks.min_elem.x;
        let max_x = self.chunks.max_elem.x;
        let min_y = self.chunks.min_elem.y;
        let max_y = self.chunks.max_elem.y;
        let mut k = 0;
        for (x, y) in (min_y..=max_y).flat_map(|y| (min_x..=max_x).map(move |x| (x, y))) {
            let chunk_index = MatrixIndex { x, y };
            if self[chunk_index].is_some() {
                k += 1;
                if k > upto {
                    return false;
                }
                if k > *first_chunk {
                    self.simulate_chunk(voxel_world, chunk_index, modified, frame, rand, bern, uni);
                }
                if tmr.elapsed().as_micros() > TIME_ALLOCATED {
                    *first_chunk = k;
                    return true;
                }
            }
        }
        false
    }
    pub fn simulate_chunk(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
        chunk_index: MatrixIndex,
        modified: &mut ChunkMapModified,
        frame: FrameCount,
        rand: &mut SmallRng,
        bern: Bernoulli,
        uni: Uniform<usize>,
    ) {
        for y in (0..=(CHUNK_HEIGHT - 1).strict_cast::<ChunkIndexType>()).rev() {
            for x in 0..=(CHUNK_WIDTH - 1).strict_cast::<ChunkIndexType>() {
                let index = FullIndex {
                    cell_index: MatrixIndex {
                        x: if y.is_multiple_of(2) {
                            x
                        } else {
                            (CHUNK_WIDTH - 1).strict_cast::<ChunkIndexType>() - x
                        },
                        y,
                    },
                    chunk_index,
                };
                self.simulate_cell(voxel_world, index, modified, frame, rand, bern, uni);
            }
        }
    }
    pub fn simulate_cell(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
        index: FullIndex,
        modified: &mut ChunkMapModified,
        frame: FrameCount,
        rand: &mut SmallRng,
        bern: Bernoulli,
        uni: Uniform<usize>,
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
                let check = if rand.sample(bern) {
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
                self.swap_from_list(voxel_world, index, modified, &check);
            }
            CellType::Granular => {
                let check = if rand.sample(bern) {
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
                self.swap_from_list(voxel_world, index, modified, &check);
            }
            CellType::Gas => {
                let check = [match rand.sample(uni) {
                    0 => index + (0, 1) - (1, 0),
                    1 => index + (0, 1),
                    2 => index + (1, 1),
                    3 => index - (1, 0),
                    4 => index,
                    5 => index + (1, 0),
                    _ => unreachable!(),
                }];
                self.swap_from_list(voxel_world, index, modified, &check);
            }
            _ => {}
        }
    }
    pub fn swap_from_list(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
        index: FullIndex,
        modified: &mut ChunkMapModified,
        check: &[FullIndex],
    ) {
        for swap_index in check.iter().copied() {
            if self.try_swap(voxel_world, index, swap_index) {
                modified.visual_modified = true;
                return;
            }
        }
    }
    pub fn try_swap(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
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
    pub fn swap(
        &mut self,
        voxel_world: &mut VoxelChunkMap,
        index: FullIndex,
        swap_index: FullIndex,
    ) {
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
            }
        } else if let [Some(from), Some(to)] = self
            .chunks
            .matrix
            .get_disjoint_mut([index.chunk_index, swap_index.chunk_index])
        {
            mem::swap(&mut from[index.cell_index], &mut to[swap_index.cell_index]);
        }
    }
}

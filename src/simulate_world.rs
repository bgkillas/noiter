use crate::cell::CellType;
use crate::chunk_map::{ChunkMap, FullIndex};
use crate::matrix::MatrixIndex;
use bevy::prelude::ResMut;
use rand::rngs::SmallRng;
use rand::{RngExt as _, make_rng};
use std::mem;
use std::range::RangeInclusive;
pub fn simulate_world(mut world: ResMut<ChunkMap>) {
    world.simulate();
}
impl ChunkMap {
    pub fn simulate(&mut self) {
        let mut rand: SmallRng = make_rng();
        let min_x = self.chunks.min_elem.x;
        let max_x = self.chunks.max_elem.x;
        let min_y = self.chunks.min_elem.y;
        let max_y = self.chunks.max_elem.y;
        for (x, y) in (min_y..=max_y).flat_map(|y| (min_x..=max_x).map(move |x| (x, y))) {
            let chunk_index = MatrixIndex { x, y };
            if self[chunk_index].is_some() {
                self.simulate_chunk(chunk_index, &mut rand);
            }
        }
    }
    pub fn simulate_chunk(&mut self, chunk_index: MatrixIndex, rand: &mut SmallRng) {
        for i in RangeInclusive::from(0..=u16::MAX) {
            let cell_index = MatrixIndex::from(i);
            let index = FullIndex {
                cell_index,
                chunk_index,
            };
            self.simulate_cell(index, rand);
        }
    }
    pub fn simulate_cell(&mut self, index: FullIndex, rand: &mut SmallRng) {
        let Some(cell) = self.get(index) else { return };
        match cell.cell_type {
            CellType::Liquid => {
                let check = if rand.random_bool(0.5) {
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
                self.swap_from_list(index, check);
            }
            CellType::Gas => {
                let check = [match rand.random_range(0..=8) {
                    0 => index + (0, 1) - (1, 0),
                    1 => index + (0, 1),
                    2 => index + (1, 1),
                    3 => index - (1, 0),
                    4 => index,
                    5 => index + (1, 0),
                    6 => index - (1, 1),
                    7 => index - (0, 1),
                    8 => index - (0, 1) + (1, 0),
                    _ => unreachable!(),
                }];
                self.swap_from_list(index, check);
            }
            _ => {}
        }
    }
    pub fn swap_from_list<const N: usize>(&mut self, index: FullIndex, check: [FullIndex; N]) {
        for swap_index in check {
            if self.try_swap(index, swap_index) {
                self.any_modified = true;
                return;
            }
        }
    }
    pub fn try_swap(&mut self, index: FullIndex, swap_index: FullIndex) -> bool {
        if let Some(cell) = self.get(index)
            && let Some(other) = self.get(swap_index)
            && cell.can_move(other)
        {
            self.swap(index, swap_index);
            true
        } else {
            false
        }
    }
    pub fn swap(&mut self, index: FullIndex, swap_index: FullIndex) {
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

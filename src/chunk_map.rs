use crate::cell::Cell;
use crate::chunk::Chunk;
use crate::matrix::{Matrix, MatrixBounded, MatrixIndex};
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH};
use bevy::ecs::resource::Resource;
use bevy::prelude::Entity;
#[derive(Resource, Default)]
pub struct ChunkMap {
    pub chunks: MatrixBounded<Box<Chunk>>,
    pub modified: Matrix<bool>,
    pub collider_entities: Matrix<Option<Entity>>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_endian = "big")]
pub struct CellIndex {
    pub chunk_index: MatrixIndex,
    pub cell_index: MatrixIndex,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_endian = "little")]
pub struct CellIndex {
    pub cell_index: MatrixIndex,
    pub chunk_index: MatrixIndex,
}
impl From<u32> for CellIndex {
    fn from(value: u32) -> Self {
        let width = (CHUNK_WIDTH * CHUNK_HEIGHT).strict_cast::<u32>();
        Self {
            cell_index: MatrixIndex::from((value % width).strict_cast::<u16>()),
            chunk_index: MatrixIndex::from((value / width).strict_cast::<u16>()),
        }
    }
}
impl From<(u16, u16)> for CellIndex {
    fn from((x, y): (u16, u16)) -> Self {
        let index_a = MatrixIndex::from(x);
        let index_b = MatrixIndex::from(y);
        let chunk_index = MatrixIndex {
            x: index_a.y,
            y: index_b.y,
        };
        let cell_index = MatrixIndex {
            x: index_a.x,
            y: index_b.x,
        };
        Self {
            cell_index,
            chunk_index,
        }
    }
}
impl ChunkMap {
    #[must_use]
    pub fn get(&self, index: CellIndex) -> Option<&Cell> {
        self.chunks[index.chunk_index]
            .as_ref()
            .map(|c| &c.cells[index.cell_index])
    }
    #[must_use]
    pub fn get_mut(&mut self, index: CellIndex) -> Option<&mut Cell> {
        self.chunks[index.chunk_index]
            .as_mut()
            .map(|c| &mut c.cells[index.cell_index])
    }
}

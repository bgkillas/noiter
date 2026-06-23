use crate::cell::Cell;
use crate::chunk::Chunk;
use crate::matrix::{MatrixBounded, MatrixIndex};
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH};
use bevy::ecs::resource::Resource;
#[derive(Resource, Default)]
pub struct ChunkMap {
    pub chunks: MatrixBounded<Box<Chunk>>,
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
            cell_index: (value % width).strict_cast::<u16>().into(),
            chunk_index: (value / width).strict_cast::<u16>().into(),
        }
    }
}
impl ChunkMap {
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
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
        self.chunks[chunk_index]
            .as_ref()
            .map(|c| &c.cells[cell_index])
    }
}

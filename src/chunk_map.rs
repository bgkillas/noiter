use crate::chunk::Chunk;
use crate::matrix::{Matrix, MatrixBounded, MatrixIndex};
use crate::{CHUNK_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH};
use bevy::ecs::resource::Resource;
#[derive(Resource, Default)]
pub struct ChunkMap {
    pub chunks: MatrixBounded<Box<Chunk>>,
    pub modified: Matrix<bool>,
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
        let width = (CHUNK_MAP_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT).strict_cast::<u32>();
        Self {
            cell_index: (value % width).strict_cast::<u16>().into(),
            chunk_index: (value / width).strict_cast::<u16>().into(),
        }
    }
}

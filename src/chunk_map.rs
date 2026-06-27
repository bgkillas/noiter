use crate::cell::Cell;
use crate::chunk::{Chunk, VoxelChunk};
use crate::matrix::{MatrixBounded, MatrixIndex};
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, ChunkIndexType};
use bevy::ecs::resource::Resource;
use std::ops::{Add, Index, IndexMut, Sub};
#[derive(Resource)]
pub struct ChunkMapModified {
    pub visual_modified: bool,
}
impl Default for ChunkMapModified {
    fn default() -> Self {
        Self {
            visual_modified: true,
        }
    }
}
#[derive(Resource, Default)]
pub struct ChunkMap {
    pub chunks: MatrixBounded<Chunk>,
}
#[derive(Resource, Default)]
pub struct VoxelChunkMap {
    pub chunks: MatrixBounded<VoxelChunk>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(target_endian = "big")]
pub struct CellIndex {
    pub chunk_index: MatrixIndex,
    pub cell_index: MatrixIndex,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(target_endian = "little")]
pub struct FullIndex {
    pub cell_index: MatrixIndex,
    pub chunk_index: MatrixIndex,
}
impl Add<(ChunkIndexType, ChunkIndexType)> for FullIndex {
    type Output = Self;
    fn add(self, (x, y): (ChunkIndexType, ChunkIndexType)) -> Self::Output {
        let (cx, x_overflowed) = self.cell_index.x.overflowing_add(x);
        let (cy, y_overflowed) = self.cell_index.y.overflowing_add(y);
        Self {
            cell_index: MatrixIndex { x: cx, y: cy },
            chunk_index: MatrixIndex {
                x: self.chunk_index.x + u8::from(x_overflowed),
                y: self.chunk_index.y + u8::from(y_overflowed),
            },
        }
    }
}
impl Sub<(ChunkIndexType, ChunkIndexType)> for FullIndex {
    type Output = Self;
    fn sub(self, (x, y): (ChunkIndexType, ChunkIndexType)) -> Self::Output {
        let (cx, x_overflowed) = self.cell_index.x.overflowing_sub(x);
        let (cy, y_overflowed) = self.cell_index.y.overflowing_sub(y);
        Self {
            cell_index: MatrixIndex { x: cx, y: cy },
            chunk_index: MatrixIndex {
                x: self.chunk_index.x - u8::from(x_overflowed),
                y: self.chunk_index.y - u8::from(y_overflowed),
            },
        }
    }
}
impl From<u32> for FullIndex {
    fn from(value: u32) -> Self {
        let width = (CHUNK_WIDTH * CHUNK_HEIGHT).strict_cast::<u32>();
        Self {
            cell_index: MatrixIndex::from((value % width).strict_cast::<u16>()),
            chunk_index: MatrixIndex::from((value / width).strict_cast::<u16>()),
        }
    }
}
impl From<(u16, u16)> for FullIndex {
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
    pub fn get(&self, index: FullIndex) -> Option<&Cell> {
        self.chunks[index.chunk_index]
            .as_ref()
            .map(|c| &c[index.cell_index])
    }
    #[must_use]
    pub fn get_mut(&mut self, index: FullIndex) -> Option<&mut Cell> {
        self.chunks[index.chunk_index]
            .as_mut()
            .map(|c| &mut c[index.cell_index])
    }
}
impl Index<MatrixIndex> for ChunkMap {
    type Output = Option<Chunk>;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.chunks[index]
    }
}
impl IndexMut<MatrixIndex> for ChunkMap {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.chunks[index]
    }
}

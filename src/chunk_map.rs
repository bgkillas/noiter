use crate::cell::{Cell, CellId};
use crate::chunk::{Chunk, VoxelChunk};
use crate::matrix::{MatrixBounded, MatrixIndex};
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, ChunkIndexType};
use bevy::ecs::resource::Resource;
use std::ops::{Add, Deref, DerefMut, Index, IndexMut, Sub};
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
    pub fn load(&mut self, voxel_chunk_map: &mut VoxelChunkMap, index: MatrixIndex) {
        if self[index].is_none() {
            self.insert(voxel_chunk_map, index, Chunk::new(|_| 0));
        }
    }
    pub fn set(&mut self, voxel_chunk_map: &mut VoxelChunkMap, index: FullIndex, id: CellId) {
        if let Some(cell) = self.get_mut(index) {
            let old = cell.is_collider();
            cell.into(id);
            let new = cell.is_collider();
            if new != old {
                voxel_chunk_map.add_voxel(index, new);
            }
        }
    }
    pub fn insert(
        &mut self,
        voxel_chunk_map: &mut VoxelChunkMap,
        index: MatrixIndex,
        (chunk, voxel_chunk): (Chunk, VoxelChunk),
    ) {
        self.chunks.insert(index, chunk);
        voxel_chunk_map.chunks.insert(index, voxel_chunk);
    }
    pub fn remove(&mut self, voxel_chunk_map: &mut VoxelChunkMap, index: MatrixIndex) {
        self.chunks.remove(index);
        voxel_chunk_map.chunks.remove(index);
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
impl Index<MatrixIndex> for VoxelChunkMap {
    type Output = Option<VoxelChunk>;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.chunks[index]
    }
}
impl IndexMut<MatrixIndex> for VoxelChunkMap {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.chunks[index]
    }
}
impl VoxelChunkMap {
    pub fn add_voxel(&mut self, index: FullIndex, into: bool) {
        if let Some(chunk) = &mut self[index.chunk_index] {
            chunk.add_voxel(index.cell_index, into);
        }
    }
}
impl VoxelChunk {
    pub fn add_voxel(&mut self, index: MatrixIndex, into: bool) {
        self.voxels_modified = true;
        self.shape
            .make_mut()
            .as_voxels_mut()
            .unwrap()
            .set_voxel(index.into(), into);
        if into {
            self.voxels += 1;
        } else {
            self.voxels -= 1;
        }
    }
}
impl Deref for ChunkMap {
    type Target = MatrixBounded<Chunk>;
    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}
impl DerefMut for ChunkMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chunks
    }
}
impl Deref for VoxelChunkMap {
    type Target = MatrixBounded<VoxelChunk>;
    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}
impl DerefMut for VoxelChunkMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chunks
    }
}

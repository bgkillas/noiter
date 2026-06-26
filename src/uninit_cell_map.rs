use crate::chunk_map::{ChunkMap, FullIndex};
use crate::matrix::Matrix;
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH};
use std::mem::MaybeUninit;
use uninit_map::uninit_map::UninitMap;
pub struct CellMap<T> {
    chunk_index: Matrix<MaybeUninit<usize>>,
    map: UninitMap<T>,
}
impl<T> CellMap<T> {
    #[must_use]
    pub fn new(world: &ChunkMap) -> Self {
        let mut chunk_index = Matrix::uninit();
        for (i, (m, _)) in world.chunks.iter().enumerate() {
            chunk_index[m].write(i);
        }
        Self {
            chunk_index,
            map: UninitMap::new(CHUNK_WIDTH * CHUNK_HEIGHT * world.chunks.len),
        }
    }
    #[must_use]
    pub fn get(&self, index: FullIndex) -> Option<&T> {
        let chunk = unsafe { self.chunk_index[index.chunk_index].assume_init() };
        self.map.get(
            chunk * CHUNK_WIDTH * CHUNK_HEIGHT + index.cell_index.flatten().strict_cast::<usize>(),
        )
    }
    pub fn insert(&mut self, index: FullIndex, val: T) {
        let chunk = unsafe { self.chunk_index[index.chunk_index].assume_init() };
        self.map.insert(
            chunk * CHUNK_WIDTH * CHUNK_HEIGHT + index.cell_index.flatten().strict_cast::<usize>(),
            val,
        );
    }
}

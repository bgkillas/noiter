use crate::CHUNK_MAP_HEIGHT;
use crate::cell::Cell;
use crate::matrix::{Matrix, MatrixIndex};
use bevy::prelude::Entity;
use std::ops::{Index, IndexMut};
pub struct Chunk {
    pub cells: Box<Matrix<Cell>>,
    pub modified: bool,
    pub collider: Option<Entity>,
}
impl Chunk {
    pub fn new(mut f: impl FnMut(MatrixIndex) -> usize) -> Self {
        let mut cells = Box::<Matrix<Cell>>::new_uninit();
        unsafe {
            for y in 0..CHUNK_MAP_HEIGHT {
                for x in 0..CHUNK_MAP_HEIGHT {
                    let cell_index = MatrixIndex::new(x, y);
                    *cells.as_mut_ptr().as_mut().unwrap().index_mut(cell_index) =
                        Cell::new(f(cell_index));
                }
            }
            Self {
                cells: cells.assume_init(),
                modified: true,
                collider: None,
            }
        }
    }
}
impl Index<MatrixIndex> for Chunk {
    type Output = Cell;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.cells[index]
    }
}
impl IndexMut<MatrixIndex> for Chunk {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

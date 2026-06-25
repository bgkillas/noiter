use crate::CHUNK_MAP_HEIGHT;
use crate::cell::Cell;
use crate::matrix::{Matrix, MatrixIndex};
use std::ops::{Index, IndexMut};
#[repr(transparent)]
pub struct Chunk {
    pub cells: Matrix<Cell>,
}
impl Chunk {
    pub fn new(mut f: impl FnMut(MatrixIndex) -> usize) -> Box<Self> {
        let mut ret = Box::<Self>::new_uninit();
        unsafe {
            for y in 0..CHUNK_MAP_HEIGHT {
                for x in 0..CHUNK_MAP_HEIGHT {
                    let cell_index = MatrixIndex::new(x, y);
                    (*ret.as_mut_ptr()).cells.elems[y][x] = Cell::new(f(cell_index));
                }
            }
            ret.assume_init()
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

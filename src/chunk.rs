use crate::cell::Cell;
use crate::matrix::Matrix;
#[derive(Default)]
pub struct Chunk {
    pub cells: Matrix<Cell>,
}

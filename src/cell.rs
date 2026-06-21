use crate::matrix::MatrixIndex;
pub type CellColor = [u8; 4];
#[derive(Default)]
pub struct Cell {
    pub color: CellColor,
}
#[derive(Clone, Copy)]
pub struct CellIndex {
    pub chunk_index: MatrixIndex,
    pub cell_index: MatrixIndex,
}
impl Cell {
    pub fn new(color: CellColor) -> Cell {
        Self { color }
    }
}

pub type CellColor = [u8; 3];
pub struct Cell {
    pub color: CellColor,
}
impl Cell {
    pub fn new(color: CellColor) -> Cell {
        Self { color }
    }
}

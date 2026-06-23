pub type CellColor = [u8; 4];
pub struct Cell {
    pub color: CellColor,
}
impl Cell {
    #[must_use]
    pub fn new(color: CellColor) -> Cell {
        Self { color }
    }
}

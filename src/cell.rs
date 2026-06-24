#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct CellColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl From<CellColor> for [u8; 4] {
    fn from(value: CellColor) -> Self {
        [value.r, value.g, value.b, value.a]
    }
}
impl CellColor {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    #[must_use]
    pub fn is_air(&self) -> bool {
        *self == Self::AIR
    }
    pub const AIR: Self = Self::new(0, 0, 0, 0);
}
pub struct Cell {
    pub color: CellColor,
}
impl Cell {
    #[must_use]
    pub fn new(color: CellColor) -> Self {
        Self { color }
    }
    #[must_use]
    pub fn is_air(&self) -> bool {
        self.color.is_air()
    }
}

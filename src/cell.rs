use crate::cells::CELLS;
use std::mem;
pub struct CellData {
    pub id: usize,
    pub color: CellColor,
    pub name: &'static str,
    pub type_data: CellDataType,
    pub cell_type: CellType,
}
pub struct CellDataPhysics {}
pub struct CellDataStatic {}
pub struct CellDataLiquid {}
pub struct CellDataGas {}
pub struct CellDataFire {}
pub enum CellDataType {
    Physics(CellDataPhysics),
    Static(CellDataStatic),
    Liquid(CellDataLiquid),
    Gas(CellDataGas),
    Fire(CellDataFire),
    Air,
}
#[derive(PartialEq, Clone, Copy)]
pub enum CellType {
    Physics,
    Static,
    Liquid,
    Gas,
    Fire,
    Air,
}
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct CellColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
    pub fn is_air(self) -> bool {
        self == Self::AIR
    }
    #[must_use]
    pub fn as_u32(self) -> u32 {
        unsafe { mem::transmute(self) }
    }
    pub const AIR: Self = Self::new(0, 0, 0, 0);
}
pub struct Cell {
    pub color: CellColor,
    pub cell_data: &'static CellData,
    pub cell_type: CellType,
}
impl Cell {
    #[must_use]
    pub fn new(id: usize) -> Self {
        let cell_data = &CELLS[id];
        Self {
            color: cell_data.color,
            cell_data,
            cell_type: cell_data.cell_type,
        }
    }
    #[must_use]
    pub fn is_air(&self) -> bool {
        self.color.is_air()
    }
}

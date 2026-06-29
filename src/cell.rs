use crate::cells::CELLS;
use std::{mem, ptr};
#[derive(Debug)]
pub struct CellData {
    pub id: usize,
    pub color: CellColor,
    pub name: &'static str,
    pub type_data: CellDataType,
}
#[derive(Debug)]
pub struct CellDataPhysics {}
#[derive(Debug)]
pub struct CellDataStatic {}
#[derive(Debug)]
pub struct CellDataGranular {}
#[derive(Debug)]
pub struct CellDataLiquid {}
#[derive(Debug)]
pub struct CellDataGas {}
#[derive(Debug)]
pub struct CellDataFire {}
#[derive(Debug)]
pub enum CellDataType {
    Physics(CellDataPhysics),
    Static(CellDataStatic),
    Granular(CellDataGranular),
    Liquid(CellDataLiquid),
    Gas(CellDataGas),
    Fire(CellDataFire),
    Air,
}
#[derive(PartialEq, Clone, Copy)]
pub enum CellType {
    Physics,
    Static,
    Granular,
    Liquid,
    Gas,
    Fire,
    Air,
}
impl CellDataType {
    #[must_use]
    pub fn cell_type(&self) -> CellType {
        match self {
            CellDataType::Physics(_) => CellType::Physics,
            CellDataType::Static(_) => CellType::Static,
            CellDataType::Granular(_) => CellType::Granular,
            CellDataType::Liquid(_) => CellType::Liquid,
            CellDataType::Gas(_) => CellType::Gas,
            CellDataType::Fire(_) => CellType::Fire,
            CellDataType::Air => CellType::Air,
        }
    }
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
    pub const AIR: Self = Self::new(0x00, 0x00, 0x00, 0x00);
}
#[derive(Clone)]
pub struct Cell {
    pub color: CellColor,
    pub cell_data: &'static CellData,
    pub cell_type: CellType,
    pub last_changed: u8,
}
impl Cell {
    #[must_use]
    pub fn new(id: usize) -> Self {
        let cell_data = &CELLS[id];
        Self {
            color: cell_data.color,
            cell_data,
            cell_type: cell_data.type_data.cell_type(),
            last_changed: 0,
        }
    }
    #[must_use]
    pub fn is_air(&self) -> bool {
        self.color.is_air()
    }
    #[must_use]
    pub fn is_collider(&self) -> bool {
        matches!(self.cell_type, CellType::Static | CellType::Granular)
    }
    #[must_use]
    pub fn can_move(&self, other: &Self) -> bool {
        matches!(
            other.cell_type,
            CellType::Liquid | CellType::Gas | CellType::Air
        ) && !ptr::eq(self.cell_data, other.cell_data)
    }
    pub fn into(&mut self, id: usize) {
        *self = Cell::new(id);
    }
}

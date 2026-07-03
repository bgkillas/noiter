use crate::cells::CELLS;
use std::{mem, ptr};
pub type CellId = u16;
#[derive(Debug)]
pub struct CellData {
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
    pub id: CellId,
    pub cell_type: CellType,
    pub last_changed: u8,
    pub velocity: CellVelocity,
    pub friction: CellFriction,
}
#[derive(Clone, Default)]
pub struct CellVelocity {
    pub x: i8,
    pub y: i8,
}
#[derive(Clone, Default)]
pub struct CellFriction {
    pub x: u8,
    pub y: u8,
}
impl Cell {
    #[must_use]
    pub fn new(id: CellId) -> Self {
        let cell_data = &CELLS[id.strict_cast::<usize>()];
        Self {
            color: cell_data.color,
            cell_data,
            id,
            cell_type: cell_data.type_data.cell_type(),
            last_changed: 0,
            velocity: CellVelocity::default(),
            friction: CellFriction::default(),
        }
    }
    pub fn friction(&mut self, x: u8, y: u8) {
        fn run(f: &mut i8, r: &mut u8, i: u8) {
            let (n, over) = r.overflowing_shl(i.strict_cast());
            *r = n;
            if over {
                if f.is_positive() {
                    *f -= 1;
                } else {
                    *f += 1;
                }
            }
        }
        run(&mut self.velocity.x, &mut self.friction.x, x);
        run(&mut self.velocity.y, &mut self.friction.y, y);
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
    pub fn into(&mut self, id: CellId) {
        let cell_data = &CELLS[id.strict_cast::<usize>()];
        self.id = id;
        self.cell_data = cell_data;
        self.cell_type = cell_data.type_data.cell_type();
        self.color = cell_data.color;
    }
}

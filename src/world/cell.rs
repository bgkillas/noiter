use crate::cells::CELLS;
use crate::simulate_world::Direction;
use std::{mem, ptr};
pub type CellId = u16;
#[derive(Debug)]
pub struct CellData {
    pub color: CellColor,
    pub name: &'static str,
    pub density: u8,
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
#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_data: &'static CellData,
    pub last_changed: u8,
    pub velocity: CellVelocity,
    pub friction: CellFriction,
    pub gravity_part: u8,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct CellVelocity {
    pub x: i8,
    pub y: i8,
}
impl CellVelocity {
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }
    #[must_use]
    pub fn to_dir(self, direction: Direction) -> Self {
        let Self { x, y } = self;
        match direction {
            Direction::UpLeft => Self { x: y, y: -y },
            Direction::Up => Self { x, y: -y },
            Direction::UpRight => Self { x: -y, y: -y },
            Direction::Left => Self { x: -y, y: x },
            Direction::Right => Self { x: y, y: x },
            Direction::DownLeft => Self { x: y, y },
            Direction::Down => Self { x, y },
            Direction::DownRight => Self { x: -y, y },
        }
    }
}
#[derive(Clone, Default, Debug)]
pub struct CellFriction {
    pub x: u8,
    pub y: u8,
}
impl Cell {
    #[must_use]
    pub fn new(id: CellId) -> Self {
        let cell_data = &CELLS[id.strict_cast::<usize>()];
        Self {
            cell_data,
            last_changed: 0,
            velocity: CellVelocity::default(),
            friction: CellFriction::default(),
            gravity_part: 0,
        }
    }
    pub fn friction(&mut self, x: u8, y: u8) {
        fn run(f: &mut i8, r: &mut u8, i: u8) {
            let (n, over) = r.overflowing_add(i);
            *r = n;
            if over {
                if f.is_positive() {
                    *f -= 1;
                } else {
                    *f += 1;
                }
            }
        }
        if self.velocity.x != 0 {
            run(&mut self.velocity.x, &mut self.friction.x, x);
        }
        if self.velocity.y != 0 {
            run(&mut self.velocity.y, &mut self.friction.y, y);
        }
    }
    pub fn gravity(&mut self, val: u8) {
        if self.velocity.y > -4 {
            let (n, over) = self.gravity_part.overflowing_add(val);
            self.gravity_part = n;
            if over {
                self.velocity.y -= 1;
            }
        }
    }
    pub fn reverse_gravity(&mut self, val: u8) {
        if self.velocity.y < 4 {
            let (n, over) = self.gravity_part.overflowing_add(val);
            self.gravity_part = n;
            if over {
                self.velocity.y += 1;
            }
        }
    }
    #[must_use]
    pub fn is_air(&self) -> bool {
        self.color().is_air()
    }
    #[must_use]
    pub fn is_collider(&self) -> bool {
        matches!(self.cell_type(), CellType::Static | CellType::Granular)
    }
    #[must_use]
    pub fn color(&self) -> CellColor {
        self.cell_data.color
    }
    #[must_use]
    pub fn cell_type(&self) -> CellType {
        self.cell_data.type_data.cell_type()
    }
    #[must_use]
    pub fn id(&self) -> CellId {
        ((ptr::from_ref(self.cell_data).addr() - ptr::from_ref(&CELLS).addr())
            / size_of::<CellData>())
        .strict_cast()
    }
    #[must_use]
    pub fn density(&self) -> u8 {
        self.cell_data.density
    }
    #[must_use]
    pub fn can_move(&self, other: &Self) -> bool {
        self.density() > other.density()
    }
    pub fn into(&mut self, id: CellId) {
        let cell_data = &CELLS[id.strict_cast::<usize>()];
        self.cell_data = cell_data;
    }
}

use crate::matrix::MatrixIndex;
use crate::{CHUNK_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_SCALE};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Transform};
use bevy::sprite::Sprite;
pub struct Cell {
    pub entity: Entity,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_endian = "big")]
pub struct CellIndex {
    pub chunk_index: MatrixIndex,
    pub cell_index: MatrixIndex,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_endian = "little")]
pub struct CellIndex {
    pub cell_index: MatrixIndex,
    pub chunk_index: MatrixIndex,
}
impl From<u32> for CellIndex {
    fn from(value: u32) -> Self {
        let width = (CHUNK_MAP_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT).strict_cast::<u32>();
        Self {
            cell_index: (value % width).strict_cast::<u16>().into(),
            chunk_index: (value / width).strict_cast::<u16>().into(),
        }
    }
}
impl Cell {
    pub fn new(color: Color, index: CellIndex, commands: &mut Commands) -> Cell {
        let x = PIXEL_SCALE
            * ((index.chunk_index.x() * CHUNK_WIDTH + index.cell_index.x()) as f32 + 0.5);
        let y = PIXEL_SCALE
            * ((index.chunk_index.y() * CHUNK_HEIGHT + index.cell_index.y()) as f32 + 0.5);
        let entity = commands
            .spawn((
                Transform::from_xyz(x, y, 0.0),
                Sprite::from_color(color, Vec2::splat(PIXEL_SCALE)),
            ))
            .id();
        Self { entity }
    }
}

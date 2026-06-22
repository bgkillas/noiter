use crate::matrix::MatrixIndex;
use crate::{
    CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_HEIGHT, PIXEL_SCALE,
    PIXEL_WIDTH,
};
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
impl Cell {
    pub fn new(color: Color, index: CellIndex, commands: &mut Commands) -> Cell {
        let x = PIXEL_SCALE
            * (index.chunk_index.x() * CHUNK_MAP_WIDTH * CHUNK_WIDTH + index.cell_index.x()) as f32
            + 0.5;
        let y = PIXEL_SCALE
            * (index.chunk_index.y() * CHUNK_MAP_HEIGHT * CHUNK_HEIGHT + index.cell_index.y())
                as f32
            + 0.5;
        let entity = commands
            .spawn((
                Transform::from_xyz(x, y, 0.0),
                Sprite::from_color(color, Vec2::new(PIXEL_WIDTH as f32, PIXEL_HEIGHT as f32)),
            ))
            .id();
        Self { entity }
    }
}

use crate::chunk::Chunk;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use crate::{CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_SCALE};
use bevy::camera::Camera2d;
use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::prelude::{ResMut, Transform};
pub fn startup(mut commands: Commands, mut chunk_map: ResMut<ChunkMap>) {
    let x = PIXEL_SCALE * (CHUNK_WIDTH * CHUNK_MAP_WIDTH / 2) as f32;
    let y = PIXEL_SCALE * (CHUNK_HEIGHT * CHUNK_MAP_HEIGHT / 2) as f32;
    commands.spawn((Camera2d, Transform::from_xyz(x, y, 0.0)));
    for chunk_y_index in CHUNK_MAP_HEIGHT / 2..=CHUNK_MAP_HEIGHT / 2 {
        for chunk_x_index in CHUNK_MAP_WIDTH / 2..=CHUNK_MAP_WIDTH / 2 {
            let chunk_index = MatrixIndex::new(chunk_x_index, chunk_y_index);
            let chunk = Chunk::new(
                chunk_index,
                |cell_index| match cell_index.x % 4 + cell_index.y % 4 {
                    0 => Color::linear_rgb(255.0, 85.0, 85.0),
                    1 => Color::linear_rgb(85.0, 255.0, 85.0),
                    2 => Color::linear_rgb(85.0, 85.0, 255.0),
                    3 => Color::linear_rgb(255.0, 85.0, 255.0),
                    4 => Color::linear_rgb(255.0, 255.0, 85.0),
                    5 => Color::linear_rgb(85.0, 255.0, 255.0),
                    6 => Color::linear_rgb(255.0, 255.0, 255.0),
                    _ => Color::linear_rgb(85.0, 85.0, 85.0),
                },
                &mut commands,
            );
            chunk_map.chunks.insert(chunk_index, chunk);
        }
    }
}

use crate::chunk::Chunk;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use crate::{CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_SCALE};
use bevy::camera::{Camera2d, OrthographicProjection, Projection};
use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::prelude::{ResMut, Transform};
pub fn startup(mut commands: Commands, mut chunk_map: ResMut<ChunkMap>) {
    let x = PIXEL_SCALE * (CHUNK_WIDTH * CHUNK_MAP_WIDTH / 2) as f32;
    let y = PIXEL_SCALE * (CHUNK_HEIGHT * CHUNK_MAP_HEIGHT / 2) as f32;
    let mut ortho = OrthographicProjection::default_2d();
    ortho.scale = PIXEL_SCALE / 8.0;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(ortho),
        Transform::from_xyz(x, y, 0.0),
    ));
    for chunk_y_index in CHUNK_MAP_HEIGHT / 2 - 1..=CHUNK_MAP_HEIGHT / 2 {
        for chunk_x_index in CHUNK_MAP_WIDTH / 2 - 1..=CHUNK_MAP_WIDTH / 2 {
            let chunk_index = MatrixIndex::new(chunk_x_index, chunk_y_index);
            let chunk = Chunk::new(
                chunk_index,
                |cell_index| match cell_index.x % 4 + cell_index.y % 4 {
                    0 => Color::linear_rgb(1.0, 1.0 / 3.0, 1.0 / 3.0),
                    1 => Color::linear_rgb(1.0 / 3.0, 1.0, 1.0 / 3.0),
                    2 => Color::linear_rgb(1.0 / 3.0, 1.0 / 3.0, 1.0),
                    3 => Color::linear_rgb(1.0, 1.0 / 3.0, 1.0),
                    4 => Color::linear_rgb(1.0, 1.0, 1.0 / 3.0),
                    5 => Color::linear_rgb(1.0 / 3.0, 1.0, 1.0),
                    6 => Color::linear_rgb(1.0, 1.0, 1.0),
                    _ => Color::linear_rgb(1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0),
                },
                &mut commands,
            );
            chunk_map.chunks.insert(chunk_index, chunk);
        }
    }
}

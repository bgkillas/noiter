use crate::chunk::Chunk;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use crate::{CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_SCALE};
use bevy::camera::Camera2d;
use bevy::ecs::system::Commands;
use bevy::prelude::{ResMut, Transform};
pub fn startup(mut commands: Commands, mut chunk_map: ResMut<ChunkMap>) {
    let x = PIXEL_SCALE * (CHUNK_WIDTH * CHUNK_MAP_WIDTH / 2) as f32;
    let y = PIXEL_SCALE * (CHUNK_HEIGHT * CHUNK_MAP_HEIGHT / 2) as f32;
    commands.spawn((Camera2d, Transform::from_xyz(x, y, 0.0)));
    for chunk_y_index in CHUNK_MAP_HEIGHT / 2 - 1..=CHUNK_MAP_HEIGHT / 2 + 1 {
        for chunk_x_index in CHUNK_MAP_WIDTH / 2 - 1..=CHUNK_MAP_WIDTH / 2 + 1 {
            let chunk_index = MatrixIndex {
                x: chunk_x_index.strict_cast(),
                y: chunk_y_index.strict_cast(),
            };
            let mut chunk = Chunk::default();
            for (cell_index, cell) in chunk.cells.iter_mut_enumerate() {
                cell.color = match cell_index.x % 4 + cell_index.y % 4 {
                    0 => [255, 85, 85, 0],
                    1 => [85, 255, 85, 0],
                    2 => [85, 85, 255, 0],
                    3 => [255, 85, 255, 0],
                    4 => [255, 255, 85, 0],
                    5 => [85, 255, 255, 0],
                    6 => [255, 255, 255, 0],
                    _ => [85, 85, 85, 0],
                };
            }
            chunk_map.chunks.insert(chunk_index, chunk);
        }
    }
}

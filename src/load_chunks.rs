use crate::chunk_map::ChunkMap;
use crate::world_image::PixelLength;
use bevy::camera::Camera2d;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Res, ResMut, Single, Transform, With};
use bevy::window::Window;
pub fn load_chunks(
    mut world: ResMut<ChunkMap>,
    camera: Single<&Transform, With<Camera2d>>,
    window: Single<&Window>,
    pixel_length: Res<PixelLength>,
) {
    let rect = Rect::from_center_size(
        Vec2::new(camera.translation.x, camera.translation.y),
        Vec2::new(
            4.0 * window.resolution.width() / (**pixel_length as f32),
            4.0 * window.resolution.height() / (**pixel_length as f32),
        ),
    );
}

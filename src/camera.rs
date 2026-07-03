use crate::PIXEL_SCALE;
use crate::world_data::WorldModified;
use crate::world_image::{PixelLength, WorldImage, WorldImageHandle, resize_world};
use bevy::asset::Assets;
use bevy::camera::{Camera2d, Projection};
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, PopulatedMessageReader, Res, ResMut, Single, Transform, With};
use bevy::window::{Window, WindowResized};
pub fn align_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mut resize_reader: PopulatedMessageReader<WindowResized>,
) {
    if let Some(size) = resize_reader.read().last() {
        let width = size.width / 2.0;
        let height = size.height / 2.0;
        camera.translation.x =
            ((camera.translation.x - width) / PIXEL_SCALE).round() * PIXEL_SCALE + width;
        camera.translation.y =
            ((camera.translation.y - height) / PIXEL_SCALE).round() * PIXEL_SCALE + height;
    }
}
pub fn move_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    pixel_length: Res<PixelLength>,
) {
    let move_rate = if kb_input.pressed(KeyCode::ShiftLeft) {
        8.0 * PIXEL_SCALE / **pixel_length as f32
    } else {
        4.0 * PIXEL_SCALE / **pixel_length as f32
    };
    if kb_input.pressed(KeyCode::KeyW) {
        camera.translation.y += move_rate;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        camera.translation.y -= move_rate;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        camera.translation.x -= move_rate;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        camera.translation.x += move_rate;
    }
}
pub fn zoom_camera(
    mut camera: Single<&mut Projection, With<Camera2d>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut pixel_length: ResMut<PixelLength>,
    window: Single<&Window>,
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
    mut world_image: Single<&mut WorldImage>,
    mut modified: ResMut<WorldModified>,
) {
    match (
        kb_input.pressed(KeyCode::KeyQ),
        kb_input.pressed(KeyCode::KeyE),
    ) {
        (true, false) if **pixel_length != 2 => {
            **pixel_length -= 1;
        }
        (false, true) => {
            **pixel_length += 1;
        }
        _ => return,
    }
    let Projection::Orthographic(ortho) = camera.as_mut() else {
        unreachable!()
    };
    ortho.scale = PIXEL_SCALE / **pixel_length as f32;
    resize_world(
        window.resolution.width() as u32,
        window.resolution.height() as u32,
        &world_image_handle,
        &mut images,
        &mut world_image,
        **pixel_length,
        &mut modified,
    );
}

use crate::PIXEL_SCALE;
use bevy::camera::Camera2d;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MessageReader, Res, Single, Transform, With};
use bevy::window::WindowResized;
const CAMERA_MOVE_RATE_FAST: f32 = PIXEL_SCALE * 2.0;
const CAMERA_MOVE_RATE: f32 = PIXEL_SCALE;
pub fn align_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mut resize_reader: MessageReader<WindowResized>,
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
) {
    let move_rate = if kb_input.pressed(KeyCode::ShiftLeft) {
        CAMERA_MOVE_RATE_FAST
    } else {
        CAMERA_MOVE_RATE
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

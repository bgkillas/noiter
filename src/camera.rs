use crate::PIXEL_SCALE;
use bevy::camera::Camera2d;
use bevy::ecs::resource::Resource;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res, Single, Transform, Vec3, With};
use bevy::window::Window;
const CAMERA_MOVE_RATE_FAST: f32 = PIXEL_SCALE * 2.0;
const CAMERA_MOVE_RATE: f32 = PIXEL_SCALE;
#[derive(Resource)]
pub struct LastCameraPos {
    pub pos: Vec3,
}
pub fn move_camera(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
) {
    let width = window.resolution.width() / 2.0;
    let height = window.resolution.height() / 2.0;
    camera.translation.x =
        ((camera.translation.x - width) / PIXEL_SCALE).round() * PIXEL_SCALE + width;
    camera.translation.y =
        ((camera.translation.y - height) / PIXEL_SCALE).round() * PIXEL_SCALE + height;
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

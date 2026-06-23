use crate::PIXEL_SCALE;
use bevy::camera::Camera2d;
use bevy::ecs::resource::Resource;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res, Single, Transform, Vec3, With};
const CAMERA_MOVE_RATE_FAST: f32 = PIXEL_SCALE;
const CAMERA_MOVE_RATE: f32 = PIXEL_SCALE / 2.0;
#[derive(Resource)]
pub struct LastCameraPos {
    pub pos: Vec3,
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

use crate::PIXEL_SCALE;
use crate::chunk_map::{ChunkMap, FullIndex, VoxelChunkMap};
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::prelude::{GlobalTransform, KeyCode, MouseButton, Res, ResMut, Single, With};
use bevy::window::Window;
pub fn spawn_cells(
    pointer: Res<ButtonInput<MouseButton>>,
    kb: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
    camera_single: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut world: ResMut<ChunkMap>,
    mut voxel_world: ResMut<VoxelChunkMap>,
) {
    let (camera, camera_transform) = camera_single.into_inner();
    let left = pointer.pressed(MouseButton::Left);
    let mid = pointer.pressed(MouseButton::Middle);
    let right = pointer.pressed(MouseButton::Right);
    let shift = kb.pressed(KeyCode::ShiftLeft);
    if (left || mid || right)
        && let Some(pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        let px = (pos.x / PIXEL_SCALE).floor();
        let py = (pos.y / PIXEL_SCALE).floor();
        for (x, y) in shapes::circle::Circle::new(px as usize, py as usize, 8) {
            let index = FullIndex::from((x.strict_cast::<u16>(), y.strict_cast::<u16>()));
            world.set(
                &mut voxel_world,
                index,
                if left {
                    if shift { 6 } else { 9 }
                } else if mid {
                    3
                } else {
                    2
                },
            );
        }
    }
}

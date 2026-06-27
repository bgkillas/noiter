use crate::PIXEL_SCALE;
use crate::chunk_map::{ChunkMap, FullIndex, VoxelChunkMap};
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::prelude::{GlobalTransform, Local, MouseButton, Res, ResMut, Single, With};
use bevy::window::Window;
pub fn spawn_cells(
    pointer: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_single: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut world: ResMut<ChunkMap>,
    mut voxel_world: ResMut<VoxelChunkMap>,
    mut pressed: Local<bool>,
) {
    let (camera, camera_transform) = camera_single.into_inner();
    let left = pointer.pressed(MouseButton::Left);
    let mid = pointer.pressed(MouseButton::Middle);
    let right = pointer.pressed(MouseButton::Right);
    if left || mid || right {
        if !*pressed
            && let Some(pos) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            let px = (pos.x / PIXEL_SCALE).floor();
            let py = (pos.y / PIXEL_SCALE).floor();
            for (x, y) in shapes::circle::Circle::new(px as usize, py as usize, 8) {
                let index = FullIndex::from((x.strict_cast::<u16>(), y.strict_cast::<u16>()));
                world.load(&mut voxel_world, index.chunk_index);
                world.set(
                    &mut voxel_world,
                    index,
                    if left {
                        9
                    } else if mid {
                        3
                    } else {
                        2
                    },
                );
            }
        }
        *pressed = true;
    } else {
        *pressed = false;
    }
}

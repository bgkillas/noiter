use crate::PIXEL_SCALE;
use crate::circle::Circle;
use crate::world_data::{FullIndex, VoxelWorld, World};
use crate::world_image::PixelLength;
use bevy::camera::{Camera, Camera2d};
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{
    Commands, Component, Entity, GlobalTransform, KeyCode, MouseButton, Res, ResMut, Single,
    Transform, With,
};
use bevy::sprite::{Anchor, Text2d};
use bevy::text::{FontSize, TextFont};
use bevy::window::Window;
pub fn spawn_cells(
    pointer: Res<ButtonInput<MouseButton>>,
    kb: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
    camera_single: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut world: ResMut<World>,
    mut voxel_world: ResMut<VoxelWorld>,
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
        for (x, y) in Circle::new(px as u16, py as u16, 8) {
            let index = FullIndex::from((x, y));
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
#[derive(Component)]
pub struct DiagnosticUi;
pub fn pointer_diagnostic(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
    camera_single: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    world: Res<World>,
    ui: Option<Single<(Entity, &mut Transform, &mut Text2d), With<DiagnosticUi>>>,
    pixel_length: Res<PixelLength>,
) {
    let (camera, camera_transform) = camera_single.into_inner();
    let shift = kb.pressed(KeyCode::ShiftLeft);
    let ctrl = kb.pressed(KeyCode::ControlLeft);
    if (shift || ctrl)
        && let Some(pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        let px = (pos.x / PIXEL_SCALE).floor() as u16;
        let py = (pos.y / PIXEL_SCALE).floor() as u16;
        let index = FullIndex::from((px, py));
        let new = Text2d(if ctrl && let Some(cell) = world.get(index) {
            format!("{} {index}", cell.cell_data.name)
        } else {
            format!("{index}")
        });
        let scale = PIXEL_SCALE / (**pixel_length) as f32;
        let mut transform = Transform::from_xyz(pos.x + 10.0 * scale, pos.y - 10.0 * scale, 0.0);
        transform.scale = Vec3::splat(scale);
        if let Some((_, mut trans, mut text)) = ui.map(Single::into_inner) {
            *trans = transform;
            *text = new;
        } else {
            commands.spawn((
                transform,
                new,
                TextFont::from_font_size(FontSize::Px(16.0)),
                Anchor::TOP_LEFT,
                DiagnosticUi,
            ));
        }
    } else if let Some((ent, _, _)) = ui.map(Single::into_inner) {
        commands.entity(ent).despawn();
    }
}

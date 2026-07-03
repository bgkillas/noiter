use crate::PIXEL_LENGTH;
use crate::camera::align_camera;
use crate::collider_world::update_colliders;
use crate::load_chunks::load_chunks;
use crate::simulate_world::simulate_world;
use crate::world_data::{VoxelWorld, World, WorldModified};
use crate::world_image::{PixelLength, display_world, on_resize_world};
use bevy::app::{App, FixedUpdate, Plugin, Update};
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
pub const BACKGROUND_COLOR: u32 = 0x96b7_ddff;
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default());
        app.insert_resource(VoxelWorld::default());
        app.insert_resource(WorldModified::default());
        app.insert_resource(ClearColor(Color::srgba_u32(BACKGROUND_COLOR)));
        app.add_systems(
            Update,
            (update_colliders, (on_resize_world, display_world).chain()),
        );
        app.add_systems(
            FixedUpdate,
            (align_camera, load_chunks, simulate_world).chain(),
        );
        app.insert_resource(PixelLength(PIXEL_LENGTH));
    }
}

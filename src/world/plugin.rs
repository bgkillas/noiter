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
use bevy::ecs::change_detection::MaybeLocation;
use bevy::ecs::schedule::IntoScheduleConfigs as _;
use bevy::prelude::Resource;
use bevy::ptr::OwningPtr;
pub const BACKGROUND_COLOR: u32 = 0x96b7_ddff;
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    #[unsafe(no_mangle)]
    fn build(&self, app: &mut App) {
        unsafe {
            insert_boxed_resource(app, World::in_place());
            insert_boxed_resource(app, VoxelWorld::in_place());
        }
        app.init_resource::<WorldModified>();
        app.insert_resource(ClearColor(Color::srgba_u32(BACKGROUND_COLOR)));
        app.insert_resource(PixelLength(PIXEL_LENGTH));
        app.add_systems(
            Update,
            (update_colliders, (on_resize_world, display_world).chain()),
        );
        app.add_systems(
            FixedUpdate,
            (align_camera, load_chunks, simulate_world).chain(),
        );
    }
}
fn insert_boxed_resource<T: Resource>(app: &mut App, boxed: Box<T>) {
    let ptr = Box::into_non_null(boxed);
    let owned = unsafe { OwningPtr::new(ptr.cast()) };
    let component_id = app
        .world_mut()
        .components_registrator()
        .register_component::<T>();
    unsafe {
        app.world_mut()
            .insert_resource_by_id(component_id, owned, MaybeLocation::caller());
    }
}

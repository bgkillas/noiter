use bevy::ecs::resource::Resource;
use bevy::prelude::Vec3;
#[derive(Resource)]
pub struct LastCameraPos {
    pub pos: Vec3,
}

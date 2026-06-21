use bevy::camera::Camera2d;
use bevy::ecs::system::Commands;
pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

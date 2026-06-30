use crate::load_chunks::WORLD_FOLDER;
use crate::world_image::{WorldImage, WorldImageHandle};
use crate::{
    APP_NAME, CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_LENGTH,
    PIXEL_SCALE,
};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::camera::{Camera2d, OrthographicProjection, Projection};
use bevy::ecs::system::Commands;
use bevy::image::Image;
use bevy::math::Vec3;
use bevy::platform::dirs::preferences_dir;
use bevy::prelude::{ResMut, Transform};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Sprite;
use std::fs;
pub fn startup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let x = PIXEL_SCALE * (CHUNK_WIDTH * CHUNK_MAP_WIDTH / 2) as f32;
    let y = PIXEL_SCALE * (CHUNK_HEIGHT * CHUNK_MAP_HEIGHT / 2) as f32;
    let mut ortho = OrthographicProjection::default_2d();
    ortho.scale = PIXEL_SCALE / PIXEL_LENGTH as f32;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(ortho),
        Transform::from_xyz(x, y, 0.0),
    ));
    let image = Image::new(
        Extent3d {
            width: 0,
            height: 0,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0; 0],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let handle = images.add(image);
    commands.spawn((
        WorldImage::default(),
        Sprite::from_image(handle.clone()),
        Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::splat(PIXEL_SCALE)),
    ));
    commands.insert_resource(WorldImageHandle(handle));
    let Some(pref) = preferences_dir() else {
        return;
    };
    let folder_name = pref.join(APP_NAME).join(WORLD_FOLDER);
    let _ = fs::remove_dir_all(&folder_name);
    let _ = fs::create_dir_all(folder_name);
}

use crate::PIXEL_LENGTH;
use crate::chunk_map::ChunkMap;
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::camera::Camera2d;
use bevy::image::Image;
use bevy::prelude::{
    Component, MessageReader, Res, ResMut, Resource, Single, Transform, With, Without,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::WindowResized;
#[derive(Component)]
pub struct WorldImage;
#[derive(Resource)]
pub struct WorldImageHandle {
    pub handle: Handle<Image>,
}
#[unsafe(no_mangle)]
pub fn display_world(
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
    mut world_image: Single<&mut Transform, (With<WorldImage>, Without<Camera2d>)>,
    camera: Single<&Transform, (With<Camera2d>, Without<WorldImage>)>,
    world: Res<ChunkMap>,
) {
    world_image.translation = camera.translation;
    let mut image = images.get_mut(&world_image_handle.handle).unwrap();
    let data = image.data.as_mut().unwrap();
    let (chunks, _) = data.as_chunks_mut();
    for (i, c) in chunks.iter_mut().enumerate() {
        *c = match i % 4 {
            0 => [255, 85, 85, 255],
            1 => [85, 255, 85, 255],
            2 => [85, 85, 255, 255],
            _ => [255, 85, 255, 255],
        };
    }
}
pub fn resize_world(
    mut resize_reader: MessageReader<WindowResized>,
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(size) = resize_reader.read().last() {
        let mut image = images.get_mut(&world_image_handle.handle).unwrap();
        let image_width = size.width as u32 / PIXEL_LENGTH.strict_cast::<u32>() + 2;
        let image_height = size.height as u32 / PIXEL_LENGTH.strict_cast::<u32>() + 2;
        *image = Image::new(
            Extent3d {
                width: image_width,
                height: image_height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![
                u8::MAX;
                image_width.strict_cast::<usize>() * image_height.strict_cast::<usize>() * 4
            ],
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
    }
}

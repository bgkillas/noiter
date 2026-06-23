use crate::chunk_map::ChunkMap;
use crate::{PIXEL_LENGTH, PIXEL_SCALE};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::camera::Camera2d;
use bevy::image::Image;
use bevy::prelude::{
    Component, MessageReader, Res, ResMut, Resource, Single, Transform, With, Without,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::WindowResized;
#[derive(Component, Default)]
pub struct WorldImage {
    pub width: u16,
    pub height: u16,
}
#[derive(Resource)]
pub struct WorldImageHandle {
    pub handle: Handle<Image>,
}
#[unsafe(no_mangle)]
pub fn display_world(
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
    world_image: Single<(&mut Transform, &WorldImage), Without<Camera2d>>,
    camera: Single<&Transform, (With<Camera2d>, Without<WorldImage>)>,
    world: Res<ChunkMap>,
) {
    let (mut transform, world_image) = world_image.into_inner();
    let px = (camera.translation.x / PIXEL_SCALE).floor();
    transform.translation.x = px * PIXEL_SCALE;
    let py = (camera.translation.y / PIXEL_SCALE).floor();
    transform.translation.y = py * PIXEL_SCALE;
    let mut image = images.get_mut(&world_image_handle.handle).unwrap();
    let data = image.data.as_mut().unwrap();
    let (chunks, _) = data.as_chunks_mut::<4>();
    let sx = px as u16 - world_image.width / 2;
    let ex = px as u16 + world_image.width / 2;
    let sy = py as u16 - world_image.height / 2;
    let ey = py as u16 + world_image.height / 2;
    for (c, (x, y)) in chunks
        .iter_mut()
        .zip((sy..ey).rev().flat_map(|y| (sx..ex).map(move |x| (x, y))))
    {
        *c = if let Some(cell) = world.get(x, y) {
            cell.color
        } else {
            [0, 0, 0, 0]
        };
    }
}
pub fn resize_world(
    mut resize_reader: MessageReader<WindowResized>,
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
    mut world_image: Single<&mut WorldImage>,
) {
    if let Some(size) = resize_reader.read().last() {
        let mut image = images.get_mut(&world_image_handle.handle).unwrap();
        let image_width = size.width as u32 / PIXEL_LENGTH.strict_cast::<u32>() + 2;
        let image_height = size.height as u32 / PIXEL_LENGTH.strict_cast::<u32>() + 3;
        world_image.width = image_width.strict_cast::<u16>();
        world_image.height = image_height.strict_cast::<u16>();
        *image = Image::new(
            Extent3d {
                width: image_width,
                height: image_height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![
                0;
                world_image.width.strict_cast::<usize>()
                    * world_image.height.strict_cast::<usize>()
                    * 4
            ],
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
    }
}

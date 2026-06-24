use crate::PIXEL_SCALE;
use crate::cell::CellColor;
use crate::chunk_map::{CellIndex, ChunkMap};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::camera::Camera2d;
use bevy::image::Image;
use bevy::prelude::{
    Component, Deref, DerefMut, MessageReader, Res, ResMut, Resource, Single, Transform, With,
    Without,
};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::WindowResized;
#[derive(Resource, Deref, DerefMut)]
pub struct PixelLength(pub u32);
#[derive(Component, Default)]
pub struct WorldImage {
    pub width: u16,
    pub height: u16,
}
#[derive(Resource, Deref, DerefMut)]
pub struct WorldImageHandle(pub Handle<Image>);
pub fn display_world(
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
    world_image: Single<(&mut Transform, &WorldImage), Without<Camera2d>>,
    camera: Single<&Transform, (With<Camera2d>, Without<WorldImage>)>,
    world: Res<ChunkMap>,
) {
    let (mut transform, world_image_dim) = world_image.into_inner();
    let px = (camera.translation.x / PIXEL_SCALE).floor();
    transform.translation.x = px * PIXEL_SCALE;
    let py = (camera.translation.y / PIXEL_SCALE).floor();
    transform.translation.y = py * PIXEL_SCALE;
    let mut image = images.get_mut(&**world_image_handle).unwrap();
    let data = image.data.as_mut().unwrap();
    let (chunks, _) = data.as_chunks_mut::<4>();
    let sx = px as u16 - world_image_dim.width.div_floor(2);
    let ex = px as u16 + world_image_dim.width.div_ceil(2);
    let sy = py as u16 - world_image_dim.height.div_floor(2);
    let ey = py as u16 + world_image_dim.height.div_ceil(2);
    for (c, (x, y)) in chunks
        .iter_mut()
        .zip((sy..ey).rev().flat_map(|y| (sx..ex).map(move |x| (x, y))))
    {
        let idx = CellIndex::from((x, y));
        *c = <[u8; 4]>::from(if let Some(cell) = world.get(idx) {
            cell.color
        } else {
            CellColor::AIR
        });
    }
}
pub fn on_resize_world(
    mut resize_reader: MessageReader<WindowResized>,
    world_image_handle: Res<WorldImageHandle>,
    mut images: ResMut<Assets<Image>>,
    mut world_image: Single<&mut WorldImage>,
    pixel_length: Res<PixelLength>,
) {
    if let Some(size) = resize_reader.read().last() {
        resize_world(
            size.width as u32,
            size.height as u32,
            &world_image_handle,
            &mut images,
            &mut world_image,
            **pixel_length,
        );
    }
}
pub fn resize_world(
    width: u32,
    height: u32,
    world_image_handle: &Handle<Image>,
    images: &mut Assets<Image>,
    world_image: &mut WorldImage,
    pixel_length: u32,
) {
    let mut image = images.get_mut(world_image_handle).unwrap();
    let image_width = (width.div_ceil(pixel_length) + 2).next_multiple_of(2);
    let image_height = (height.div_ceil(pixel_length) + 2).next_multiple_of(2);
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

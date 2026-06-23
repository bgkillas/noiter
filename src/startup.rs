use crate::chunk::Chunk;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use crate::world_image::{WorldImage, WorldImageHandle};
use crate::{
    CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_LENGTH, PIXEL_SCALE,
};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::camera::{Camera2d, OrthographicProjection, Projection};
use bevy::ecs::system::Commands;
use bevy::image::Image;
use bevy::math::Vec3;
use bevy::prelude::{ResMut, Transform};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Sprite;
pub fn startup(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut images: ResMut<Assets<Image>>,
) {
    let x = PIXEL_SCALE * (CHUNK_WIDTH * CHUNK_MAP_WIDTH / 2) as f32;
    let y = PIXEL_SCALE * (CHUNK_HEIGHT * CHUNK_MAP_HEIGHT / 2) as f32;
    let mut ortho = OrthographicProjection::default_2d();
    ortho.scale = PIXEL_SCALE / PIXEL_LENGTH as f32;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(ortho),
        Transform::from_xyz(x, y, 0.0),
    ));
    for chunk_y_index in CHUNK_MAP_HEIGHT / 2 - 1..=CHUNK_MAP_HEIGHT / 2 {
        for chunk_x_index in CHUNK_MAP_WIDTH / 2 - 1..=CHUNK_MAP_WIDTH / 2 {
            let chunk_index = MatrixIndex::new(chunk_x_index, chunk_y_index);
            let chunk = Chunk::new(|cell_index| match cell_index.x % 4 + cell_index.y % 4 {
                0 => [255, 85, 85, 255],
                1 => [85, 255, 85, 255],
                2 => [85, 85, 255, 255],
                3 => [255, 85, 255, 255],
                4 => [255, 255, 85, 255],
                5 => [85, 255, 255, 255],
                6 => [255, 255, 255, 255],
                _ => [85, 85, 85, 255],
            });
            chunk_map.chunks.insert(chunk_index, chunk);
        }
    }
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
        Transform::from_scale(Vec3::splat(PIXEL_SCALE)),
    ));
    commands.insert_resource(WorldImageHandle(handle));
}

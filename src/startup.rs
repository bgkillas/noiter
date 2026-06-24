use crate::cell::CellColor;
use crate::chunk::Chunk;
use crate::chunk_map::{CellIndex, ChunkMap};
use crate::matrix::MatrixIndex;
use crate::world_image::{WorldImage, WorldImageHandle};
use crate::{
    CHUNK_HEIGHT, CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, PIXEL_LENGTH, PIXEL_SCALE,
};
use avian2d::prelude::{Collider, GravityScale, RigidBody, SleepingDisabled};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::camera::{Camera2d, OrthographicProjection, Projection};
use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::image::Image;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{ResMut, Transform};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Sprite;
use shapes::circumference::Circumference;
use shapes::octant::octant;
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
    let x0 = CHUNK_WIDTH * CHUNK_MAP_WIDTH / 2;
    let y0 = CHUNK_HEIGHT * CHUNK_MAP_HEIGHT / 2;
    for chunk_y_index in CHUNK_MAP_HEIGHT / 2 - 2..=CHUNK_MAP_HEIGHT / 2 + 1 {
        for chunk_x_index in CHUNK_MAP_WIDTH / 2 - 2..=CHUNK_MAP_WIDTH / 2 + 1 {
            let chunk_index = MatrixIndex::new(chunk_x_index, chunk_y_index);
            let chunk = Chunk::new(|cell_index| {
                if y0.abs_diff(
                    cell_index.y.strict_cast::<usize>()
                        + CHUNK_HEIGHT * chunk_index.y.strict_cast::<usize>(),
                ) > x0.abs_diff(
                    cell_index.x.strict_cast::<usize>()
                        + CHUNK_WIDTH * chunk_index.x.strict_cast::<usize>(),
                ) {
                    CellColor::AIR
                } else {
                    match cell_index.x % 4 + cell_index.y % 4 {
                        0 => CellColor::new(255, 85, 85, 255),
                        1 => CellColor::new(85, 255, 85, 255),
                        2 => CellColor::new(85, 85, 255, 255),
                        3 => CellColor::new(255, 85, 255, 255),
                        4 => CellColor::new(255, 255, 85, 255),
                        5 => CellColor::new(85, 255, 255, 255),
                        6 => CellColor::new(255, 255, 255, 255),
                        _ => CellColor::new(85, 85, 85, 255),
                    }
                }
            });
            chunk_map.modified[chunk_index] = true;
            chunk_map.chunks.insert(chunk_index, chunk);
        }
    }
    for r in 0..=128 {
        for (dx, dy) in Circumference::new(r) {
            octant(x0, y0, dx, dy, |_, x, y| {
                if let Some(c) =
                    chunk_map.get_mut(CellIndex::from((x.strict_cast(), y.strict_cast())))
                {
                    c.color = match r % 8 {
                        0 => CellColor::new(255, 85, 85, 255),
                        1 => CellColor::new(85, 255, 85, 255),
                        2 => CellColor::new(85, 85, 255, 255),
                        3 => CellColor::new(255, 85, 255, 255),
                        4 => CellColor::new(255, 255, 85, 255),
                        5 => CellColor::new(85, 255, 255, 255),
                        6 => CellColor::new(255, 255, 255, 255),
                        _ => CellColor::new(85, 85, 85, 255),
                    };
                }
            });
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
    commands.spawn((
        RigidBody::Dynamic,
        GravityScale(4.0 * PIXEL_SCALE),
        SleepingDisabled,
        Collider::rectangle(8.0 * PIXEL_SCALE, 8.0 * PIXEL_SCALE),
        Sprite::from_color(Color::WHITE, Vec2::splat(8.0 * PIXEL_SCALE)),
        Transform::from_xyz(x, y + PIXEL_SCALE * 3.0 * CHUNK_HEIGHT as f32, 0.0),
    ));
    commands.insert_resource(WorldImageHandle(handle));
}

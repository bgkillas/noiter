use crate::cell::Cell;
use crate::chunk::Chunk;
use crate::chunk_map::{ChunkMap, FullIndex, VoxelChunkMap};
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
    mut voxel_chunk_map: ResMut<VoxelChunkMap>,
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
            chunk_map.insert(
                &mut voxel_chunk_map,
                chunk_index,
                Chunk::new(|cell_index| {
                    if y0.abs_diff(
                        cell_index.y.strict_cast::<usize>()
                            + CHUNK_HEIGHT * chunk_index.y.strict_cast::<usize>(),
                    ) > x0.abs_diff(
                        cell_index.x.strict_cast::<usize>()
                            + CHUNK_WIDTH * chunk_index.x.strict_cast::<usize>(),
                    ) {
                        0
                    } else {
                        (cell_index.x % 5 + cell_index.y % 5 + 1).strict_cast()
                    }
                }),
            );
        }
    }
    for r in 0..=128 {
        for (dx, dy) in Circumference::new(r) {
            octant(x0, y0, dx, dy, |_, x, y| {
                let index = FullIndex::from((x.strict_cast(), y.strict_cast()));
                if let Some(chunk) = &mut chunk_map.chunks[index.chunk_index]
                    && let Some(voxel_chunk) = &mut voxel_chunk_map.chunks[index.chunk_index]
                {
                    let cell = Cell::new(r % 9 + 1);
                    if cell.is_collider() {
                        voxel_chunk
                            .shape
                            .make_mut()
                            .as_voxels_mut()
                            .unwrap()
                            .set_voxel(index.cell_index.into(), true);
                        voxel_chunk.voxels += 1;
                    } else if chunk[index.cell_index].is_collider() {
                        voxel_chunk
                            .shape
                            .make_mut()
                            .as_voxels_mut()
                            .unwrap()
                            .set_voxel(index.cell_index.into(), false);
                        voxel_chunk.voxels -= 1;
                    }
                    chunk[index.cell_index] = cell;
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
        Transform::from_xyz(0.0, 0.0, -1.0).with_scale(Vec3::splat(PIXEL_SCALE)),
    ));
    for i in 2..8 {
        commands.spawn((
            RigidBody::Dynamic,
            GravityScale(4.0 * PIXEL_SCALE),
            SleepingDisabled,
            Collider::rectangle(8.0 * PIXEL_SCALE, 8.0 * PIXEL_SCALE),
            Sprite::from_color(Color::WHITE, Vec2::splat(8.0 * PIXEL_SCALE)),
            Transform::from_xyz(x, y + PIXEL_SCALE * i as f32 * CHUNK_HEIGHT as f32, 0.0),
        ));
    }
    commands.insert_resource(WorldImageHandle(handle));
}

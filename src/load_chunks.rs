use crate::chunk::Chunk;
use crate::pixel_run::PixelRunBuilder;
use crate::world_data::{VoxelWorld, World};
use crate::world_image::WorldImage;
use crate::{APP_NAME, CHUNK_HEIGHT, CHUNK_WIDTH, ChunkIndexType, PIXEL_SCALE};
use bevy::camera::Camera2d;
use bevy::platform::dirs::preferences_dir;
use bevy::prelude::{Commands, ResMut, Single, Transform, With};
use rand::distr::Uniform;
use rand::rngs::SmallRng;
use rand::{RngExt as _, make_rng};
use std::fs;
use std::fs::OpenOptions;
pub const WORLD_FOLDER: &str = "world";
pub fn load_chunks(
    mut world: ResMut<World>,
    mut voxel_world: ResMut<VoxelWorld>,
    mut commands: Commands,
    camera: Single<&Transform, With<Camera2d>>,
    world_image: Single<&WorldImage>,
) {
    let Some(pref) = preferences_dir() else {
        return;
    };
    let folder_name = pref.join(APP_NAME).join(WORLD_FOLDER);
    if !fs::exists(&folder_name).is_ok_and(|b| b) {
        return;
    }
    let px = (camera.translation.x / PIXEL_SCALE).floor() as u16;
    let py = (camera.translation.y / PIXEL_SCALE).floor() as u16;
    let min_x = px - world_image.width;
    let min_y = py - world_image.height;
    let max_x = px + world_image.width;
    let max_y = py + world_image.height;
    let min_cx = (min_x as usize / CHUNK_WIDTH).strict_cast::<ChunkIndexType>();
    let min_cy = (min_y as usize / CHUNK_HEIGHT).strict_cast::<ChunkIndexType>();
    let max_cx = (max_x as usize)
        .div_ceil(CHUNK_WIDTH)
        .strict_cast::<ChunkIndexType>();
    let max_cy = (max_y as usize)
        .div_ceil(CHUNK_HEIGHT)
        .strict_cast::<ChunkIndexType>();
    world.par_take_zip_if(
        &mut voxel_world,
        &mut commands,
        |i| i.x < min_cx || i.y < min_cy || i.x > max_cx || i.y > max_cy,
        |iter| {
            let mut pixel_run = PixelRunBuilder::default();
            let mut ret = Vec::with_capacity(iter.len());
            for (i, chunk, voxel) in iter {
                if let Some(ent) = voxel.collider {
                    ret.push(ent);
                }
                pixel_run.write_chunk(chunk);
                pixel_run.finish();
                let file_name = folder_name.join(format!("{}x{}", i.x, i.y));
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file_name)
                    .unwrap();
                pixel_run.write(file);
                pixel_run.clear();
            }
            ret
        },
    );
    let uniform = Uniform::new(2, 5).unwrap();
    world.par_iter_none_in_range(&mut voxel_world, min_cx, min_cy, max_cx, max_cy, |iter| {
        let mut small_rng: SmallRng = make_rng();
        let mut pixel_run = PixelRunBuilder::default();
        let mut vec = Vec::with_capacity(iter.len());
        for idx in iter.iter().copied() {
            if let Ok(file) = OpenOptions::new()
                .read(true)
                .create(false)
                .open(folder_name.join(format!("{}x{}", idx.x, idx.y)))
            {
                pixel_run.read(file);
                let mut iter = pixel_run.pixel_run().iter();
                vec.push((idx, Chunk::new(|_| iter.next().unwrap())));
                pixel_run.clear();
            } else {
                vec.push((
                    idx,
                    Chunk::new(|_| {
                        if small_rng.random_bool(0.8) {
                            0
                        } else {
                            small_rng.sample(uniform)
                        }
                    }),
                ));
            }
        }
        vec
    });
}

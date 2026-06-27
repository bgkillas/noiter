use crate::chunk::Chunk;
use crate::chunk_map::{ChunkMap, ChunkMapModified, VoxelChunkMap};
use crate::matrix::MatrixIndex;
use crate::world_image::write_data;
use crate::{CHUNK_HEIGHT, CHUNK_WIDTH, ChunkIndexType};
use bevy::diagnostic::FrameCount;
use test::{Bencher, black_box};
#[bench]
fn bench_write_data(bencher: &mut Bencher) {
    let mut chunk_map = ChunkMap::default();
    let mut voxel_world = VoxelChunkMap::default();
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(0, 0), Chunk::new(|_| 1));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(1, 0), Chunk::new(|_| 1));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(0, 1), Chunk::new(|_| 1));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(1, 1), Chunk::new(|_| 1));
    let mut data = vec![[0u8; 4]; 256 * 256 * 2 * 2];
    bencher.iter(|| {
        write_data(
            black_box(&chunk_map),
            black_box(&mut data),
            black_box(0),
            black_box(256 * 2),
            black_box(256 * 2),
        )
    })
}
#[bench]
fn bench_simulate(bencher: &mut Bencher) {
    let mut chunk_map = ChunkMap::default();
    let mut voxel_world = VoxelChunkMap::default();
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(0, 0), Chunk::new(|_| 0));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(1, 0), Chunk::new(|_| 0));
    chunk_map.insert(
        &mut voxel_world,
        MatrixIndex::new(0, 1),
        Chunk::new(|index| {
            if index.x > (CHUNK_WIDTH / 2).strict_cast::<ChunkIndexType>()
                && index.y < (CHUNK_HEIGHT / 2).strict_cast::<ChunkIndexType>()
            {
                1
            } else {
                0
            }
        }),
    );
    chunk_map.insert(
        &mut voxel_world,
        MatrixIndex::new(1, 1),
        Chunk::new(|index| {
            if index.x < (CHUNK_WIDTH / 2).strict_cast::<ChunkIndexType>()
                && index.y < (CHUNK_HEIGHT / 2).strict_cast::<ChunkIndexType>()
            {
                2
            } else {
                0
            }
        }),
    );
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(2, 0), Chunk::new(|_| 9));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(2, 1), Chunk::new(|_| 0));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(2, 2), Chunk::new(|_| 0));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(0, 2), Chunk::new(|_| 0));
    chunk_map.insert(&mut voxel_world, MatrixIndex::new(1, 2), Chunk::new(|_| 0));
    let mut i = 0;
    let mut modified = ChunkMapModified::default();
    bencher.iter(|| {
        i += 1;
        const MAX: u128 = u128::MAX;
        chunk_map.simulate::<MAX>(&mut voxel_world, &mut modified, FrameCount(i), &mut 0);
    })
}

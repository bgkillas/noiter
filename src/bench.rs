use crate::chunk::Chunk;
use crate::chunk_map::{ChunkMap, VoxelChunkMap};
use crate::matrix::MatrixIndex;
use crate::simulate_world::WorldRand;
use crate::world_image::write_data;
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
    let mut i = 0;
    let mut world_rand = WorldRand::default();
    bencher.iter(|| {
        i += 1;
        chunk_map[MatrixIndex::new(0, 0)]
            .as_mut()
            .unwrap()
            .simulate(
                voxel_world[MatrixIndex::new(0, 0)].as_mut().unwrap(),
                &mut world_rand,
                FrameCount(i),
            );
    })
}

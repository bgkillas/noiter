use crate::chunk::Chunk;
use crate::matrix::MatrixIndex;
use crate::pixel_run::PixelRunBuilder;
use crate::simulate_world::WorldRand;
use crate::world_data::{VoxelWorld, World};
use crate::world_image::write_data;
use test::{Bencher, black_box};
#[bench]
fn bench_write_data(bencher: &mut Bencher) {
    let mut world = World::default();
    let mut voxel_world = VoxelWorld::default();
    world.insert(&mut voxel_world, MatrixIndex::new(0, 0), Chunk::new(|_| 1));
    world.insert(&mut voxel_world, MatrixIndex::new(1, 0), Chunk::new(|_| 1));
    world.insert(&mut voxel_world, MatrixIndex::new(0, 1), Chunk::new(|_| 1));
    world.insert(&mut voxel_world, MatrixIndex::new(1, 1), Chunk::new(|_| 1));
    let mut data = vec![[0u8; 4]; 256 * 256 * 2 * 2];
    bencher.iter(|| {
        write_data(
            black_box(&world),
            black_box(&mut data),
            black_box(0),
            black_box(256 * 2),
            black_box(256 * 2),
        )
    })
}
#[bench]
fn bench_simulate(bencher: &mut Bencher) {
    let mut world = World::default();
    let mut voxel_world = VoxelWorld::default();
    world.insert(&mut voxel_world, MatrixIndex::new(0, 0), Chunk::new(|_| 0));
    let mut i = 0;
    let mut world_rand = WorldRand::default();
    bencher.iter(|| {
        world.simulate_chunk(&mut voxel_world, MatrixIndex::new(0, 0), &mut world_rand, i);
        i = i.wrapping_add(1);
    })
}
#[bench]
fn bench_pixel_run(bencher: &mut Bencher) {
    let mut world = World::default();
    let mut voxel_world = VoxelWorld::default();
    let mut world_rand = WorldRand::default();
    world.insert(
        &mut voxel_world,
        MatrixIndex::new(0, 0),
        Chunk::new(|_| if world_rand.half() { 1 } else { 0 }),
    );
    let mut pixel_run = PixelRunBuilder::default();
    let chunk = world[MatrixIndex::new(0, 0)].as_ref().unwrap();
    pixel_run.write_chunk(&chunk);
    pixel_run.finish();
    assert_eq!(pixel_run.pixel_run().iter().count(), 65536);
    pixel_run.clear();
    bencher.iter(|| {
        let chunk = world[MatrixIndex::new(0, 0)].as_ref().unwrap();
        pixel_run.write_chunk(&chunk);
        pixel_run.finish();
        black_box(&pixel_run);
        pixel_run.clear();
    });
}

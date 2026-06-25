use crate::chunk::Chunk;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use crate::world_image::write_data;
use test::{Bencher, black_box};
#[bench]
fn bench_write_data(bencher: &mut Bencher) {
    let mut chunk_map = ChunkMap::default();
    chunk_map
        .chunks
        .insert(MatrixIndex::new(0, 0), Chunk::new(|_| 1));
    chunk_map
        .chunks
        .insert(MatrixIndex::new(1, 0), Chunk::new(|_| 1));
    chunk_map
        .chunks
        .insert(MatrixIndex::new(0, 1), Chunk::new(|_| 1));
    chunk_map
        .chunks
        .insert(MatrixIndex::new(1, 1), Chunk::new(|_| 1));
    let mut data = vec![[0u8; 4]; 256 * 256 * 3 * 3];
    bencher.iter(|| {
        write_data(
            black_box(&chunk_map),
            black_box(&mut data),
            black_box(0),
            black_box(256 * 3),
            black_box(256 * 3),
        )
    })
}

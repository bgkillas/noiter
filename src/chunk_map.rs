use crate::chunk::Chunk;
use crate::matrix::{Matrix, MatrixBounded};
use bevy::ecs::resource::Resource;
#[derive(Resource, Default)]
pub struct ChunkMap {
    pub chunks: MatrixBounded<Box<Chunk>>,
    pub modified: Matrix<bool>,
}

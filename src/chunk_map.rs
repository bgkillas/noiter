use crate::chunk::Chunk;
use crate::matrix::{Matrix, MatrixBounded};
use bevy::ecs::resource::Resource;
use bevy::prelude::NonSendMut;
use bevy_framebuffer::pixels_impl::PixelsFrame;
#[derive(Resource, Default)]
pub struct ChunkMap {
    pub chunks: MatrixBounded<Box<Chunk>>,
    pub modified: Matrix<bool>,
}
pub fn draw_chunks(mut buffer: NonSendMut<PixelsFrame>) {}

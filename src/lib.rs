#![feature(integer_casts)]
pub mod app;
pub mod camera;
pub mod cell;
pub mod chunk;
pub mod chunk_map;
pub mod matrix;
mod startup;
mod update;
mod world_image;
pub const PIXEL_SCALE: f32 = 1.0 / 256.0;
pub const PIXEL_LENGTH: usize = 16;
pub const CHUNK_WIDTH: usize = 256;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_MAP_WIDTH: usize = 256;
pub const CHUNK_MAP_HEIGHT: usize = 256;
pub type ChunkIndexType = u8;

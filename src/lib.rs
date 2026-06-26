#![feature(integer_casts)]
#![feature(int_roundings)]
#![feature(slice_ptr_get)]
#![feature(test)]
pub mod app;
#[cfg(test)]
mod bench;
pub mod camera;
pub mod cell;
pub mod cells;
pub mod chunk;
pub mod chunk_map;
pub mod collider_world;
pub mod matrix;
pub mod simulate_world;
pub mod startup;
pub mod uninit_cell_map;
pub mod world_image;
extern crate test;
pub const PIXEL_SCALE: f32 = 1.0;
pub const PIXEL_LENGTH: u32 = 16;
pub const CHUNK_WIDTH: usize = 256;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_MAP_WIDTH: usize = 256;
pub const CHUNK_MAP_HEIGHT: usize = 256;
pub type ChunkIndexType = u8;

#![feature(integer_casts)]
#![feature(int_roundings)]
#![feature(slice_ptr_get)]
#![cfg_attr(test, feature(test))]
pub mod app;
#[cfg(test)]
mod bench;
pub mod camera;
pub mod cell;
pub mod cells;
pub mod chunk;
pub mod chunk_map;
pub mod collider_world;
pub mod load_chunks;
pub mod matrix;
pub mod pixel_run;
pub mod pointer;
pub mod simulate_world;
pub mod startup;
pub mod uninit_cell_map;
pub mod world_image;
#[cfg(test)]
extern crate test;
pub const PIXEL_SCALE: f32 = 1.0;
pub const PIXEL_LENGTH: u32 = 4;
pub const CHUNK_WIDTH: usize = 256;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_AREA: usize = CHUNK_WIDTH * CHUNK_HEIGHT;
pub const CHUNK_MAP_WIDTH: usize = 256;
pub const CHUNK_MAP_HEIGHT: usize = 256;
pub const CHUNK_MAP_AREA: usize = CHUNK_MAP_WIDTH * CHUNK_MAP_HEIGHT;
pub const CHUNK_WIDTH_LAST: ChunkIndexType = (CHUNK_WIDTH - 1) as ChunkIndexType;
pub const CHUNK_HEIGHT_LAST: ChunkIndexType = (CHUNK_HEIGHT - 1) as ChunkIndexType;
pub const CHUNK_MAP_WIDTH_LAST: ChunkIndexType = (CHUNK_MAP_WIDTH - 1) as ChunkIndexType;
pub const CHUNK_MAP_HEIGHT_LAST: ChunkIndexType = (CHUNK_MAP_HEIGHT - 1) as ChunkIndexType;
pub type ChunkIndexType = u8;
#[cfg(feature = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn wasm_hook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    app::app_run();
}

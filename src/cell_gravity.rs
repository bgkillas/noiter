use crate::CHUNK_HEIGHT;
use crate::cell::CellType;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use bevy::prelude::{Local, ResMut};
use std::mem;
use std::range::RangeInclusive;
const SECTIONS: usize = 16;
pub fn cell_gravity(mut world: ResMut<ChunkMap>, mut num: Local<usize>) {
    if *num == SECTIONS {
        *num = 0;
    }
    let min_x = world.chunks.min_elem.x;
    let max_x = world.chunks.max_elem.x;
    let min_y = world.chunks.min_elem.y;
    let max_y = world.chunks.max_elem.y;
    for (x, y) in (min_y..=max_y).flat_map(|y| (min_x..=max_x).map(move |x| (x, y))) {
        let chunk_index = MatrixIndex { x, y };
        //TODO does not support y == CHUNK_MAP_HEIGHT
        let upper_index = chunk_index + (0, 1);
        //TODO does not support y == 0
        let lower_index = chunk_index - (0, 1);
        let [upper, maybe_chunk, lower] =
            world
                .chunks
                .matrix
                .get_disjoint_mut([upper_index, chunk_index, lower_index]);
        let Some(chunk) = maybe_chunk else {
            continue;
        };
        let mut any_changed = false;
        let mut upper_any_changed = false;
        let mut lower_any_changed = false;
        for i in RangeInclusive::from(
            (*num * u16::MAX.strict_cast::<usize>() / SECTIONS).strict_cast::<u16>()
                ..=((*num + 1) * u16::MAX.strict_cast::<usize>() / SECTIONS).strict_cast::<u16>(),
        ) {
            let idx = MatrixIndex::from(i);
            match chunk[idx].cell_type {
                CellType::Liquid => {
                    if idx.y == 0 {
                        let lidx = MatrixIndex {
                            x: idx.x,
                            y: (CHUNK_HEIGHT - 1).strict_cast(),
                        };
                        if let Some(lc) = lower.as_mut()
                            && lc[lidx].cell_type == CellType::Air
                        {
                            any_changed = true;
                            lower_any_changed = true;
                            mem::swap(&mut chunk[idx], &mut lc[lidx]);
                        }
                    } else if chunk[idx - (0, 1)].cell_type == CellType::Air {
                        any_changed = true;
                        chunk.cells.swap(idx, idx - (0, 1));
                    }
                }
                CellType::Gas => {
                    if idx.y == (CHUNK_HEIGHT - 1).strict_cast() {
                        let uidx = MatrixIndex { x: idx.x, y: 0 };
                        if let Some(uc) = upper.as_mut()
                            && uc[uidx].cell_type == CellType::Air
                        {
                            any_changed = true;
                            upper_any_changed = true;
                            mem::swap(&mut chunk[idx], &mut uc[uidx]);
                        }
                    } else if chunk[idx + (0, 1)].cell_type == CellType::Air {
                        any_changed = true;
                        chunk.cells.swap(idx, idx + (0, 1));
                    }
                }
                _ => {}
            }
        }
        if any_changed {
            if upper_any_changed {
                upper.as_mut().unwrap().voxels_modified = true;
            }
            if lower_any_changed {
                lower.as_mut().unwrap().voxels_modified = true;
            }
            chunk.voxels_modified = true;
            world.any_modified = true;
        }
    }
    *num += 1;
}

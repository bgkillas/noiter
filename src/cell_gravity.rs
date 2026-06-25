use crate::CHUNK_HEIGHT;
use crate::cell::CellType;
use crate::chunk_map::ChunkMap;
use crate::matrix::MatrixIndex;
use bevy::prelude::ResMut;
use std::mem;
use std::range::RangeInclusive;
pub fn cell_gravity(mut world: ResMut<ChunkMap>) {
    for y in world.chunks.min_elem.y..=world.chunks.max_elem.y {
        for x in world.chunks.min_elem.x..=world.chunks.max_elem.x {
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
            for i in RangeInclusive::from(0u16..=u16::MAX) {
                let idx = MatrixIndex::from(i);
                match chunk[idx].cell_type {
                    CellType::Liquid => {
                        any_changed = true;
                        if idx.y == 0 {
                            let lidx = MatrixIndex {
                                x: idx.x,
                                y: (CHUNK_HEIGHT - 1).strict_cast(),
                            };
                            if let Some(lc) = lower.as_mut()
                                && lc[lidx].cell_type == CellType::Air
                            {
                                mem::swap(&mut chunk[idx], &mut lc[lidx]);
                            }
                        } else if chunk[idx - (0, 1)].cell_type == CellType::Air {
                            chunk.cells.swap(idx, idx - (0, 1));
                        }
                    }
                    CellType::Gas => {
                        any_changed = true;
                        if idx.y == (CHUNK_HEIGHT - 1).strict_cast() {
                            let uidx = MatrixIndex { x: idx.x, y: 0 };
                            if let Some(uc) = upper.as_mut()
                                && uc[uidx].cell_type == CellType::Air
                            {
                                mem::swap(&mut chunk[idx], &mut uc[uidx]);
                            }
                        } else if chunk[idx + (0, 1)].cell_type == CellType::Air {
                            chunk.cells.swap(idx, idx + (0, 1));
                        }
                    }
                    _ => {}
                }
            }
            if any_changed {
                chunk.modified = true;
                world.any_modified = true;
            }
        }
    }
}

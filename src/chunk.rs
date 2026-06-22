use crate::CHUNK_MAP_HEIGHT;
use crate::cell::{Cell, CellIndex};
use crate::matrix::{Matrix, MatrixIndex};
use bevy::color::Color;
use bevy::prelude::Commands;
#[repr(transparent)]
pub struct Chunk {
    pub cells: Matrix<Cell>,
}
impl Chunk {
    pub fn new(
        chunk_index: MatrixIndex,
        mut f: impl FnMut(MatrixIndex) -> Color,
        commands: &mut Commands,
    ) -> Box<Self> {
        let mut ret = Box::<Self>::new_uninit();
        unsafe {
            for y in 0..CHUNK_MAP_HEIGHT {
                for x in 0..CHUNK_MAP_HEIGHT {
                    let cell_index = MatrixIndex {
                        x: x.strict_cast(),
                        y: y.strict_cast(),
                    };
                    (*ret.as_mut_ptr()).cells.elems[y][x] = Cell::new(
                        f(cell_index),
                        CellIndex {
                            chunk_index,
                            cell_index,
                        },
                        commands,
                    );
                }
            }
            ret.assume_init()
        }
    }
}

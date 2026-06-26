use crate::cell::Cell;
use crate::matrix::{Matrix, MatrixIndex};
use crate::{CHUNK_MAP_HEIGHT, PIXEL_SCALE};
use avian2d::math::Vector;
use avian2d::parry::shape::{SharedShape, Voxels};
use bevy::prelude::Entity;
use std::ops::{Index, IndexMut};
pub struct Chunk {
    pub cells: Box<Matrix<Cell>>,
    pub voxels_modified: bool,
    pub collider: Option<Entity>,
    pub shape: SharedShape,
    pub voxels: usize,
}
impl Chunk {
    pub fn new(mut f: impl FnMut(MatrixIndex) -> usize) -> Self {
        let mut cells = Box::<Matrix<Cell>>::new_uninit();
        let mut shape = SharedShape::new(Voxels::new(Vector::splat(PIXEL_SCALE), &[]));
        let voxel = shape.make_mut().as_voxels_mut().unwrap();
        let mut voxels = 0;
        unsafe {
            let ptr = cells.as_mut_ptr().as_mut().unwrap();
            for y in 0..CHUNK_MAP_HEIGHT {
                for x in 0..CHUNK_MAP_HEIGHT {
                    let cell_index = MatrixIndex::new(x, y);
                    let cell = Cell::new(f(cell_index));
                    if cell.is_collider() {
                        voxels += 1;
                        voxel.set_voxel(cell_index.into(), true);
                    }
                    *ptr.index_mut(cell_index) = cell;
                }
            }
            Self {
                cells: cells.assume_init(),
                voxels_modified: true,
                collider: None,
                shape,
                voxels,
            }
        }
    }
}
impl Index<MatrixIndex> for Chunk {
    type Output = Cell;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.cells[index]
    }
}
impl IndexMut<MatrixIndex> for Chunk {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

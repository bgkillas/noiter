use crate::chunk::Chunk;
use crate::{CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, ChunkIndexType};
use std::array;
use std::hint::assert_unchecked;
use std::ops::{Index, IndexMut};
pub struct Matrix<T> {
    pub elems: [[T; CHUNK_MAP_WIDTH]; CHUNK_MAP_HEIGHT],
}
pub struct MatrixBounded<T> {
    pub matrix: Matrix<Option<T>>,
    pub len: usize,
    pub min_elem: MatrixIndex,
    pub max_elem: MatrixIndex,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_endian = "big")]
pub struct MatrixIndex {
    pub y: ChunkIndexType,
    pub x: ChunkIndexType,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg(target_endian = "little")]
pub struct MatrixIndex {
    pub x: ChunkIndexType,
    pub y: ChunkIndexType,
}
impl MatrixIndex {
    pub fn x(self) -> usize {
        self.x.strict_cast()
    }
    pub fn y(self) -> usize {
        self.y.strict_cast()
    }
    pub fn new(x: usize, y: usize) -> Self {
        unsafe {
            assert_unchecked(x < 256);
            assert_unchecked(y < 256);
        }
        Self {
            x: x.strict_cast(),
            y: y.strict_cast(),
        }
    }
}
impl From<u16> for MatrixIndex {
    fn from(value: u16) -> Self {
        let [a, b] = value.to_ne_bytes();
        Self { x: a, y: b }
    }
}
impl<T> Matrix<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elems.iter().flat_map(|elems| elems.iter())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elems.iter_mut().flat_map(|elems| elems.iter_mut())
    }
    pub fn iter_enumerate(&self) -> impl Iterator<Item = (MatrixIndex, &T)> {
        self.elems
            .as_flattened()
            .iter()
            .enumerate()
            .map(|(i, cell)| (i.strict_cast::<u16>().into(), cell))
    }
    pub fn iter_mut_enumerate(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut T)> {
        self.elems
            .as_flattened_mut()
            .iter_mut()
            .enumerate()
            .map(|(i, cell)| (i.strict_cast::<u16>().into(), cell))
    }
}
impl<T> Index<MatrixIndex> for Matrix<T> {
    type Output = T;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.elems[index.y()][index.x()]
    }
}
impl<T> IndexMut<MatrixIndex> for Matrix<T> {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.elems[index.y()][index.x()]
    }
}
impl<T: Default> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            elems: array::from_fn(|_| array::from_fn(|_| T::default())),
        }
    }
}
impl<T> Index<MatrixIndex> for MatrixBounded<T> {
    type Output = Option<T>;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.matrix[index]
    }
}
impl<T> IndexMut<MatrixIndex> for MatrixBounded<T> {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}
impl<T> Default for MatrixBounded<T> {
    fn default() -> Self {
        Self {
            matrix: Matrix::default(),
            len: 0,
            min_elem: MatrixIndex {
                x: ChunkIndexType::MAX,
                y: ChunkIndexType::MAX,
            },
            max_elem: MatrixIndex { x: 0, y: 0 },
        }
    }
}
impl MatrixBounded<Box<Chunk>> {
    #[inline]
    pub fn remove(&mut self, index: MatrixIndex) {
        if self.matrix[index].take().is_some() {
            let min_x = self.min_elem.x;
            let max_x = self.max_elem.x;
            let min_y = self.min_elem.y;
            let max_y = self.max_elem.y;
            self.min_elem.x = ChunkIndexType::MAX;
            self.min_elem.y = ChunkIndexType::MAX;
            self.max_elem.x = ChunkIndexType::MIN;
            self.max_elem.y = ChunkIndexType::MIN;
            self.len -= 1;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let i = MatrixIndex { x, y };
                    if self.matrix[i].is_some() {
                        self.min_elem.x = self.min_elem.x.min(x);
                        self.min_elem.y = self.min_elem.y.min(y);
                        self.max_elem.x = self.max_elem.x.max(x);
                        self.max_elem.y = self.max_elem.y.max(y);
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
    #[inline]
    pub fn insert(&mut self, index: MatrixIndex, chunk: Box<Chunk>) {
        if self.matrix[index].is_some() {
            unreachable!()
        } else {
            self.len += 1;
        }
        self.matrix[index] = Some(chunk);
        self.min_elem.x = self.min_elem.x.min(index.x);
        self.min_elem.y = self.min_elem.y.min(index.y);
        self.max_elem.x = self.max_elem.x.max(index.x);
        self.max_elem.y = self.max_elem.y.max(index.y);
    }
}

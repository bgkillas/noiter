use crate::{CHUNK_MAP_HEIGHT, CHUNK_MAP_WIDTH, CHUNK_WIDTH, CHUNK_WIDTH_LAST, ChunkIndexType};
use avian2d::parry::math::IVector;
use bevy::tasks::ComputeTaskPool;
use std::array;
use std::cmp::Ordering;
use std::hint::assert_unchecked;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Add, Index, IndexMut, Sub};
use std::ptr::NonNull;
use std::range::RangeFrom;
#[derive(Clone)]
pub struct Matrix<T> {
    pub elems: [[T; CHUNK_MAP_WIDTH]; CHUNK_MAP_HEIGHT],
}
#[derive(Clone)]
pub struct MatrixBounded<T> {
    pub matrix: Matrix<Option<T>>,
    pub len: usize,
    pub min_elem: MatrixIndex,
    pub max_elem: MatrixIndex,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(target_endian = "big")]
pub struct MatrixIndex {
    pub y: ChunkIndexType,
    pub x: ChunkIndexType,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(target_endian = "little")]
pub struct MatrixIndex {
    pub x: ChunkIndexType,
    pub y: ChunkIndexType,
}
impl Eq for MatrixIndex {}
impl PartialOrd for MatrixIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MatrixIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flatten().cmp(&other.flatten())
    }
}
impl From<MatrixIndex> for IVector {
    fn from(value: MatrixIndex) -> Self {
        Self {
            x: value.x.strict_cast(),
            y: value.y.strict_cast(),
        }
    }
}
impl MatrixIndex {
    #[must_use]
    pub fn x(self) -> usize {
        self.x.strict_cast()
    }
    #[must_use]
    pub fn y(self) -> usize {
        self.y.strict_cast()
    }
    #[must_use]
    pub fn flatten(self) -> u16 {
        self.y.strict_cast::<u16>() * CHUNK_WIDTH.strict_cast::<u16>() + self.x.strict_cast::<u16>()
    }
    #[must_use]
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
impl Add<(ChunkIndexType, ChunkIndexType)> for MatrixIndex {
    type Output = Self;
    fn add(self, (x, y): (ChunkIndexType, ChunkIndexType)) -> Self::Output {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}
impl Sub<(ChunkIndexType, ChunkIndexType)> for MatrixIndex {
    type Output = Self;
    fn sub(self, (x, y): (ChunkIndexType, ChunkIndexType)) -> Self::Output {
        Self {
            x: self.x - x,
            y: self.y - y,
        }
    }
}
impl From<u16> for MatrixIndex {
    fn from(value: u16) -> Self {
        let width = CHUNK_WIDTH.strict_cast::<u16>();
        Self {
            x: (value % width).strict_cast(),
            y: (value / width).strict_cast(),
        }
    }
}
impl<T> Matrix<T> {
    #[must_use]
    pub fn uninit() -> Matrix<MaybeUninit<T>> {
        Matrix {
            elems: [const { [const { MaybeUninit::uninit() }; CHUNK_MAP_WIDTH] }; CHUNK_MAP_HEIGHT],
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elems.as_flattened().iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elems.as_flattened_mut().iter_mut()
    }
    pub fn iter_enumerate(&self) -> impl Iterator<Item = (MatrixIndex, &T)> {
        self.elems
            .as_flattened()
            .iter()
            .zip(RangeFrom::from(0u16..))
            .map(|(cell, i)| (MatrixIndex::from(i), cell))
    }
    pub fn iter_mut_enumerate(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut T)> {
        self.elems
            .as_flattened_mut()
            .iter_mut()
            .zip(RangeFrom::from(0u16..))
            .map(|(cell, i)| (MatrixIndex::from(i), cell))
    }
    pub fn get_disjoint_mut<const N: usize>(&mut self, idx: [MatrixIndex; N]) -> [&mut T; N] {
        self.elems
            .as_flattened_mut()
            .get_disjoint_mut(idx.map(|i| i.flatten().strict_cast::<usize>()))
            .unwrap()
    }
    pub fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        idx: [MatrixIndex; N],
    ) -> [&mut T; N] {
        unsafe {
            self.elems
                .as_flattened_mut()
                .get_disjoint_unchecked_mut(idx.map(|i| i.flatten().strict_cast::<usize>()))
        }
    }
    pub fn swap(&mut self, from: MatrixIndex, to: MatrixIndex) {
        self.elems.as_flattened_mut().swap(
            from.flatten().strict_cast::<usize>(),
            to.flatten().strict_cast::<usize>(),
        );
    }
}
impl<T> Matrix<Option<T>> {
    pub fn get_neighbors_mut(&mut self, idx: MatrixIndex) -> [Option<&mut T>; 5] {
        let mut ret = [None, None, None, None, None];
        let mut idxs = [None, None, Some(idx), None, None];
        if idx.y != 0 {
            idxs[0] = Some(idx - (0, 1));
        }
        if idx.x != 0 {
            idxs[1] = Some(idx - (1, 0));
        }
        if idx.x != CHUNK_WIDTH_LAST {
            idxs[3] = Some(idx + (1, 0));
        }
        if idx.y != CHUNK_WIDTH_LAST {
            idxs[4] = Some(idx + (0, 1));
        }
        let arr_ptr: *mut [Option<T>] = self.elems.as_flattened_mut();
        for (i, opt_pos) in idxs.into_iter().enumerate() {
            if let Some(pos) = opt_pos {
                unsafe {
                    ret[i] = (&mut *arr_ptr
                        .get_unchecked_mut(pos.flatten().strict_cast::<usize>()))
                        .as_mut();
                }
            }
        }
        ret
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
impl<T> MatrixBounded<T> {
    pub fn remove(&mut self, index: MatrixIndex) -> T {
        if let Some(r) = self.matrix[index].take() {
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
            r
        } else {
            unreachable!()
        }
    }
    pub fn insert(&mut self, index: MatrixIndex, chunk: T) {
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
    pub fn par_iter_mut<R: Send + 'static>(
        &mut self,
        f: impl Fn(&mut dyn Iterator<Item = (MatrixIndex, &mut T)>) -> R + Send + Sync,
    ) -> Vec<R> {
        #[repr(transparent)]
        struct SendPtr<T>(NonNull<T>);
        unsafe impl<T> Send for SendPtr<T> {}
        unsafe impl<T> Sync for SendPtr<T> {}
        let task_pool = ComputeTaskPool::get();
        let chunk_size = (self.len / task_pool.thread_num()).max(1);
        let f_ref = &f;
        let mut list = Vec::with_capacity(self.len);
        list.extend(
            self.iter_mut()
                .map(|(i, c)| (i, SendPtr(NonNull::from_mut(c)))),
        );
        task_pool.scope(|scope| {
            for chunk_ptr in list.chunks_mut(chunk_size) {
                scope.spawn(async move {
                    let mut iter = chunk_ptr
                        .iter_mut()
                        .map(|(i, p)| (*i, unsafe { p.0.as_mut() }));
                    f_ref(&mut iter)
                });
            }
        })
    }
    pub fn par_iter_zip_mut<R: Send + 'static, K>(
        &mut self,
        other: &mut MatrixBounded<K>,
        f: impl Fn(&mut dyn Iterator<Item = (MatrixIndex, &mut T, &mut K)>) -> R + Send + Sync,
    ) -> Vec<R> {
        #[repr(transparent)]
        struct SendPtr<T>(NonNull<T>);
        unsafe impl<T> Send for SendPtr<T> {}
        unsafe impl<T> Sync for SendPtr<T> {}
        let task_pool = ComputeTaskPool::get();
        let chunk_size = (self.len / task_pool.thread_num()).max(1);
        let f_ref = &f;
        let mut list = Vec::with_capacity(self.len);
        list.extend(self.iter_mut().map(|(i, c)| {
            (
                i,
                SendPtr(NonNull::from_mut(c)),
                SendPtr(NonNull::from_mut(other[i].as_mut().unwrap())),
            )
        }));
        task_pool.scope(|scope| {
            for chunk_ptr in list.chunks_mut(chunk_size) {
                scope.spawn(async move {
                    let mut iter = chunk_ptr
                        .iter_mut()
                        .map(|(i, a, b)| (*i, unsafe { a.0.as_mut() }, unsafe { b.0.as_mut() }));
                    f_ref(&mut iter)
                });
            }
        })
    }
    pub fn iter(&self) -> impl Iterator<Item = (MatrixIndex, &T)> {
        (self.min_elem.y..=self.max_elem.y).flat_map(move |y| {
            (self.min_elem.x..=self.max_elem.x).filter_map(move |x| {
                let idx = MatrixIndex { x, y };
                self.matrix[idx].as_ref().map(|c| (idx, c))
            })
        })
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut T)> {
        MatrixBoundedIterMut {
            min_elem_x: self.min_elem.x,
            min_elem: self.min_elem,
            max_elem: self.max_elem,
            matrix: NonNull::new(&raw mut self.matrix).unwrap(),
            phantom: PhantomData,
        }
        .filter_map(|(i, ch)| ch.as_mut().map(|c| (i, c)))
    }
    pub fn iter_opt(&self) -> impl Iterator<Item = (MatrixIndex, &Option<T>)> {
        (self.min_elem.y..=self.max_elem.y).flat_map(move |y| {
            (self.min_elem.x..=self.max_elem.x).filter_map(move |x| {
                let idx = MatrixIndex { x, y };
                if self.matrix[idx].is_some() {
                    Some((idx, &self.matrix[idx]))
                } else {
                    None
                }
            })
        })
    }
    pub fn iter_opt_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut Option<T>)> {
        MatrixBoundedIterMut {
            min_elem_x: self.min_elem.x,
            min_elem: self.min_elem,
            max_elem: self.max_elem,
            matrix: NonNull::new(&raw mut self.matrix).unwrap(),
            phantom: PhantomData,
        }
        .filter(|(_, ch)| ch.is_some())
    }
    pub fn take_if(
        &mut self,
        cond: impl Fn(MatrixIndex) -> bool,
        mut f: impl FnMut(MatrixIndex, T),
    ) {
        for y in self.min_elem.y..=self.max_elem.y {
            for x in self.min_elem.x..=self.max_elem.x {
                let idx = MatrixIndex { x, y };
                if cond(idx) {
                    f(idx, self.remove(idx));
                }
            }
        }
    }
}
pub struct MatrixBoundedIterMut<'a, T> {
    min_elem_x: ChunkIndexType,
    min_elem: MatrixIndex,
    max_elem: MatrixIndex,
    matrix: NonNull<Matrix<Option<T>>>,
    phantom: PhantomData<&'a mut Matrix<Option<T>>>,
}
impl<'a, T> Iterator for MatrixBoundedIterMut<'a, T> {
    type Item = (MatrixIndex, &'a mut Option<T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.min_elem_x > self.max_elem.x {
            return None;
        }
        if self.min_elem.x > self.max_elem.x {
            self.min_elem.x = self.min_elem_x;
            self.min_elem.y += 1;
        }
        if self.min_elem.y > self.max_elem.y {
            return None;
        }
        let next = (self.min_elem, unsafe {
            self.matrix.as_mut().index_mut(self.min_elem)
        });
        self.min_elem.x += 1;
        Some(next)
    }
}

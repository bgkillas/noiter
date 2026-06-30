use crate::CHUNK_AREA;
use crate::cell::CellId;
use crate::chunk::Chunk;
use std::hint::assert_unchecked;
use std::mem::MaybeUninit;
use std::ops::Index;
use std::ptr;
pub struct Array<T, const N: usize> {
    pub arr: [MaybeUninit<T>; N],
    pub len: usize,
}
impl<T, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Self {
            arr: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }
}
impl<T, const N: usize> Index<usize> for Array<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            assert_unchecked(self.len < N);
        }
        if self.len > index {
            unsafe { self.arr[index].assume_init_ref() }
        } else {
            unreachable!()
        }
    }
}
impl<T, const N: usize> Array<T, N> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        unsafe {
            assert_unchecked(self.len < N);
        }
        self.arr[..self.len]
            .iter()
            .map(|e| unsafe { e.assume_init_ref() })
    }
    pub fn push(&mut self, val: T) {
        unsafe {
            assert_unchecked(self.len < N);
        }
        self.arr[self.len].write(val);
        self.len += 1;
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    #[must_use]
    pub fn slice(&self) -> &[T] {
        unsafe {
            assert_unchecked(self.len < N);
        }
        unsafe { &*(ptr::from_ref(&self.arr[..self.len]) as *const [T]) }
    }
}
#[derive(Default)]
pub struct PixelRun {
    pub arr: Array<(u16, CellId), CHUNK_AREA>,
}
impl PixelRun {
    #[must_use]
    pub fn iter(&self) -> PixelRunIter<'_> {
        PixelRunIter {
            arr: self.arr.slice(),
            current: self.arr[0].0,
            pixel: self.arr[0].1,
        }
    }
}
impl<'a> IntoIterator for &'a PixelRun {
    type Item = CellId;
    type IntoIter = PixelRunIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Default)]
pub struct PixelRunBuilder {
    pub inner: PixelRun,
    pub current: CellId,
    pub len: u16,
}
impl PixelRunBuilder {
    #[must_use]
    pub fn build(mut self) -> PixelRun {
        self.write();
        self.inner
    }
    pub fn write(&mut self) {
        if self.len != 0 {
            self.inner.arr.push((self.len, self.current));
            self.current = 0;
            self.len = 0;
        }
    }
    pub fn push(&mut self, pixel: CellId) {
        if self.current == pixel {
            self.len += 1;
        } else {
            self.write();
            self.current = pixel;
            self.len = 1;
        }
    }
    pub fn clear(&mut self) {
        self.len = 0;
        self.current = 0;
        self.inner.arr.clear();
    }
    #[unsafe(no_mangle)]
    pub fn write_chunk(&mut self, chunk: &Chunk) {
        for cell in chunk.cells.iter() {
            self.push(cell.cell_data.id);
        }
    }
}
pub struct PixelRunIter<'a> {
    arr: &'a [(u16, CellId)],
    current: u16,
    pixel: CellId,
}
impl Iterator for PixelRunIter<'_> {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.arr.is_empty() {
            None
        } else if self.current == 0 {
            self.arr = &self.arr[1..];
            self.current = self.arr[0].0;
            self.pixel = self.arr[0].1;
            self.current -= 1;
            Some(self.pixel)
        } else {
            self.current -= 1;
            Some(self.pixel)
        }
    }
}

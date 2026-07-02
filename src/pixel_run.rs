use crate::CHUNK_AREA;
use crate::cell::CellId;
use crate::chunk::Chunk;
use std::fs::File;
use std::hint::assert_unchecked;
use std::io::{Read as _, Write as _};
use std::mem::MaybeUninit;
use std::ops::Index;
use std::{ptr, slice};
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
pub struct PixelRun<'a> {
    pub arr: &'a [[u16; 2]],
}
impl<'a> PixelRun<'a> {
    #[must_use]
    pub fn iter(self) -> PixelRunIter<'a> {
        PixelRunIter {
            arr: &self.arr[1..],
            current: self.arr[0][0],
            pixel: self.arr[0][1],
            next: false,
        }
    }
}
impl<'a> IntoIterator for PixelRun<'a> {
    type Item = CellId;
    type IntoIter = PixelRunIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Default)]
pub struct PixelRunOwned {
    pub arr: Array<[u16; 2], CHUNK_AREA>,
}
impl PixelRunOwned {
    #[must_use]
    pub fn slice(&self) -> PixelRun<'_> {
        PixelRun {
            arr: self.arr.slice(),
        }
    }
}
#[derive(Default)]
pub struct PixelRunBuilder {
    pub inner: PixelRunOwned,
    pub current: CellId,
    pub len: u16,
}
impl PixelRunBuilder {
    #[must_use]
    pub fn build(&mut self) -> PixelRun<'_> {
        self.finish();
        self.pixel_run()
    }
    #[must_use]
    pub fn build_owned(mut self) -> PixelRunOwned {
        self.finish();
        self.inner
    }
    #[must_use]
    pub fn pixel_run(&self) -> PixelRun<'_> {
        PixelRun {
            arr: self.inner.arr.slice(),
        }
    }
    pub fn finish(&mut self) {
        self.inner.arr.push([self.len, self.current]);
        self.len = 0;
    }
    pub fn push(&mut self, pixel: CellId) {
        if self.current == pixel {
            self.len += 1;
        } else {
            self.finish();
            self.current = pixel;
            self.len = 0;
        }
    }
    pub fn clear(&mut self) {
        self.len = 0;
        self.inner.arr.clear();
    }
    pub fn write_chunk(&mut self, chunk: &Chunk) {
        let mut iter = chunk.cells.iter();
        self.current = iter.next().unwrap().id;
        for cell in iter {
            self.push(cell.id);
        }
    }
    pub fn write(&self, mut file: File) {
        let slice = self.pixel_run().arr;
        let u8_slice = unsafe { slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 4) };
        file.write_all(u8_slice).unwrap();
    }
    pub fn read(&mut self, mut file: File) {
        let slice = unsafe {
            slice::from_raw_parts_mut((&raw mut self.inner.arr.arr).cast(), CHUNK_AREA * 4)
        };
        self.inner.arr.len = file.read(slice).unwrap();
    }
}
impl Chunk {
    #[must_use]
    pub fn get_pixel_run(&self) -> PixelRunOwned {
        let mut pixel_run = PixelRunBuilder::default();
        pixel_run.write_chunk(self);
        pixel_run.build_owned()
    }
}
pub struct PixelRunIter<'a> {
    arr: &'a [[u16; 2]],
    current: u16,
    pixel: CellId,
    next: bool,
}
impl Iterator for PixelRunIter<'_> {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next {
            if self.arr.is_empty() {
                return None;
            }
            self.current = self.arr[0][0];
            self.pixel = self.arr[0][1];
            self.arr = &self.arr[1..];
            self.next = false;
        }
        if self.current == 0 {
            self.next = true;
        }
        self.current = self.current.saturating_sub(1);
        Some(self.pixel)
    }
}

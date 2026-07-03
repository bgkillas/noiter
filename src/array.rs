use std::hint::assert_unchecked;
use std::mem::MaybeUninit;
use std::ops::Index;
use std::{mem, ptr};
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
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        unsafe {
            assert_unchecked(self.len < N);
        }
        self.arr[..self.len]
            .iter_mut()
            .map(|e| unsafe { e.assume_init_mut() })
    }
    pub fn push(&mut self, val: T) {
        unsafe {
            assert_unchecked(self.len < N);
        }
        self.arr[self.len].write(val);
        self.len += 1;
    }
    pub fn pop(&mut self) -> T {
        self.len -= 1;
        unsafe {
            assert_unchecked(self.len < N);
        }
        unsafe { mem::replace(&mut self.arr[self.len], MaybeUninit::uninit()).assume_init() }
    }
    pub fn drain(&mut self) -> impl Iterator<Item = T> {
        unsafe {
            assert_unchecked(self.len < N);
        }
        let iter = mem::replace(&mut self.arr, [const { MaybeUninit::uninit() }; N])
            .into_iter()
            .take(self.len)
            .map(|a| unsafe { a.assume_init() });
        self.len = 0;
        iter
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
impl<T, const N: usize> Drop for Array<T, N> {
    fn drop(&mut self) {
        for elem in self.iter_mut() {
            unsafe { ptr::drop_in_place(elem) }
        }
    }
}

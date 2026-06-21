use std::array;
use std::ops::Index;
pub struct Matrix<T> {
    pub elems: [[T; 256]; 256],
}
pub struct MatrixBounded<T> {
    pub matrix: Matrix<Option<T>>,
    pub len: usize,
    pub min_elem: MatrixIndex,
    pub max_elem: MatrixIndex,
}
pub type MatrixIndex = (u8, u8);
impl<T> Index<MatrixIndex> for Matrix<T> {
    type Output = T;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.elems[index.1.strict_cast::<usize>()][index.0.strict_cast::<usize>()]
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
    type Output = T;
    fn index(&self, index: MatrixIndex) -> &Self::Output {
        self.matrix[index].as_ref().unwrap()
    }
}
impl<T> Default for MatrixBounded<T> {
    fn default() -> Self {
        Self {
            matrix: Matrix::default(),
            len: 0,
            min_elem: (255, 255),
            max_elem: (0, 0),
        }
    }
}

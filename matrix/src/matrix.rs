use crate::error::MatrixError;

/// 矩阵
#[derive(Debug)]
pub struct Matrix<T> {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) data: Vec<T>,
}

impl<T> Matrix<T> {
    /// 创建一个rows行cols列的矩阵，内部没有元素
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: Vec::with_capacity(rows * cols),
        }
    }

    /// 获取矩阵第i行j列的元素的不可变引用
    pub fn get(&self, i: usize, j: usize) -> Result<&T, MatrixError> {
        if i > self.rows || j > self.cols {
            return Err(MatrixError::IndexOutOfBounds);
        }
        Ok(&self.data[i * self.cols + j])
    }

    /// 获取矩阵第i行j列的元素的不可变引用
    pub fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut T, MatrixError> {
        todo!()
    }
    
}

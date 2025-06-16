use core::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    IndexOutOfBounds,
}

impl Error for MatrixError {}

impl Display for MatrixError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MatrixError::IndexOutOfBounds => write!(f, "index out of bounds"),
        }
    }
}

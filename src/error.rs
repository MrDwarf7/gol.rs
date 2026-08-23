use bevy::prelude::UVec2;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error handler: {0}")]
    Generic(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("grid dimensions must be non-zero, got {0:?}")]
    InvalidDimensions(UVec2),
    #[error("cell {pos:?} is outside {size:?} grid")]
    OutOfBounds { pos: UVec2, size: UVec2 },
}

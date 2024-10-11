use std::io;

use thiserror::Error;


#[derive(Debug,Error,)]
pub enum AppError {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("This platform is not supported")]
    NoSupported,
}

pub type AppResult<T> = Result<T,AppError>;
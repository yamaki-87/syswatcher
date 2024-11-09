use std::io;

use thiserror::Error;


#[derive(Debug,Error,)]
pub enum AppError {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("This platform is not supported")]
    NoSupported,
    #[error("{0}")]
    LogError(#[from] flexi_logger::FlexiLoggerError),
}

pub type AppResult<T> = Result<T,AppError>;
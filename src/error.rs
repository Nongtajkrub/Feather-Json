use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("An empty path is an invalid path.")]
    NoPathProvided,

    #[error("Invalid path to value.")]
    InvalidPath,

    #[error("Std Input Output Error: {0}")]
    StdInputOutputError(#[from] std::io::Error),
}

pub type JsonResult<T> = Result<T, JsonError>;

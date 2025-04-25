use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("An empty path is an invalid path.")]
    NoPathProvided,

    #[error("Invalid path to value.")]
    InvalidPath,

    #[error("Invalid Json")]
    InvalidJson,

    #[error("")]
    InsertCantInsertIntoValue,

    #[error("Std Input Output Error: {0}")]
    StdInputOutputError(#[from] std::io::Error),
}

impl PartialEq for JsonError {
    fn eq(&self, other: &Self) -> bool {
        use JsonError::*;

        match (self, other) {
            (NoPathProvided, NoPathProvided) => true,
            (InvalidPath, InvalidPath) => true,
            (InvalidJson, InvalidJson) => true,
            (StdInputOutputError(_), StdInputOutputError(_)) => true,

            _ => false,
        }
    }
}

pub type JsonResult<T> = Result<T, JsonError>;

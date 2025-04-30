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

    #[error("Json value is not an integer.")]
    JsonValueIsNotInteger,

    #[error("Json value is not a float.")]
    JsonValueIsNotFloat,

    #[error("Json value is not a boolean.")]
    JsonValueIsNotBool,

    #[error("Json value is not a String.")]
    JsonValueIsNotString,
}

impl PartialEq for JsonError {
    fn eq(&self, other: &Self) -> bool {
        use JsonError::*;

        match (self, other) {
            (NoPathProvided, NoPathProvided) => true,
            (InvalidPath, InvalidPath) => true,
            (InvalidJson, InvalidJson) => true,
            (InsertCantInsertIntoValue, InsertCantInsertIntoValue) => true,
            (StdInputOutputError(_), StdInputOutputError(_)) => true,

            _ => false,
        }
    }
}

pub type JsonResult<T> = Result<T, JsonError>;

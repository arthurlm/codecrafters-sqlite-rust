use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SqliteError {
    #[error("I/O: {0}")]
    Io(String),

    #[error("Parse:{0}")]
    Parse(String),

    #[error("Invalid page index")]
    InvalidPageIndex,

    #[error("Invalid Sqlite schema")]
    InvalidSqliteSchema,
}

impl From<io::Error> for SqliteError {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<nom::Err<nom::error::Error<&[u8]>>> for SqliteError {
    fn from(err: nom::Err<nom::error::Error<&[u8]>>) -> Self {
        Self::Parse(err.to_string())
    }
}

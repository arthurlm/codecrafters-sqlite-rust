use std::io;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SqliteError {
    #[error("I/O: {0}")]
    Io(String),

    #[error("Parse:{0}")]
    Parse(String),

    #[error("Invalid page index")]
    InvalidPageIndex,

    #[error("Invalid page type")]
    InvalidPageType,

    #[error("Invalid Sqlite schema")]
    InvalidSqliteSchema,

    #[error("Missing primary key")]
    MissingPrimaryKey,

    #[error("Table not found")]
    TableNotFound,
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

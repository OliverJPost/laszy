use derive_more::Display;
use std::io;

#[derive(Debug, Display)]
pub enum LaszyError {
    IoError(io::Error),
    LasError(String),
    LaszyError(String),
    EmptyCloud(String),
    InvalidFileExtension(String),
}

impl From<las::Error> for LaszyError {
    fn from(error: las::Error) -> Self {
        LaszyError::LasError(error.to_string())
    }
}

impl From<io::Error> for LaszyError {
    fn from(error: io::Error) -> Self {
        LaszyError::IoError(error)
    }
}

use std::io;

#[derive(Debug)]
pub enum LaszyError {
    IoError(io::Error),
    LasError(String),
    LaszyError(String),
    EmptyCloud(String),
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

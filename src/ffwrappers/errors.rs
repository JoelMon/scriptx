/// Module that contains custom error types used by the library.
use core::fmt;

#[derive(Debug)]
pub enum Errors {
    /// Errors dealing with file read or write errors.
    FileError,
}

impl std::error::Error for Errors {}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::FileError => write!(f, "FileError:"),
        }
    }
}

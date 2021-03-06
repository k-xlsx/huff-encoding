use std::{
    io,
    fmt,
    error,
};

/// Every kind of Error returned by the program
#[derive(Clone, Debug)]
pub enum ErrorKind{
    /// When provided with invalid input 
    InvalidInput,
    /// When the file you want to decompress has an
    /// extension other than cli::EXTENSION
    UnrecognizedFormat,
    /// The file you want to decompress does
    /// not contain padding info/tree len/tree
    MissingHeaderInfo,
    /// The file you want to decompress contains
    /// invalid padding info/tree
    InvalidHeaderInfo,
    /// The provided file path points to 
    /// a directory
    NotFile,
    /// Any std::io::Error
    Io,
}

/// Error type returned by the program
#[derive(Clone)]
pub struct Error{
    pub message: String,
    pub kind: ErrorKind,
}

impl Error{
    /// Initialize a new Error with the provided message and ErrorKind 
    pub fn new(message: String, kind: ErrorKind) -> Self{
        Error{
            message,
            kind,
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Error as fmt::Display>::fmt(self, f)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(e.to_string(), ErrorKind::Io)
    }
}

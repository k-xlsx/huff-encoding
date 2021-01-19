use std::{
    process,
    fmt,
    io,
    error,
};


/// Every kind of Error returned by the program
#[derive(Clone, Debug)]
pub enum ErrorKind{
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
#[derive(Clone, Debug)]
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

    /// Prints the error to `stderr` and exits with a status of `1`
    pub fn exit(&self) -> ! {
        eprintln!("Error: {}", self.message);
        process::exit(1);  
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(String::from(e.to_string()), ErrorKind::Io)
    }
}

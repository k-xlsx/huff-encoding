// when i have time:
// TODO: add optional multithreading
// TODO: verbose option
// TODO: TESTS

/// Functions parsing and processing args
pub mod cli;
/// error returned by the program
pub mod error;
/// Functions reading file, compressing/decompressing them, 
/// and writing the results to file
mod comp;
/// Various utility functions
mod utils;

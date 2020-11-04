mod file;
mod utils;
mod huff_structs;

pub use crate::huff_structs::{
    HuffLeaf, 
    HuffBranch, 
    HuffTree, 
    HuffCode,
    ByteFreqs,
};

pub use crate::file::{
    write_hfe,
    threaded_write_hfe,
    read_hfe,
    compress,
    threaded_compress,
    decompress,
};

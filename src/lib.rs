pub mod file;
pub mod huff_structs;

mod utils;


pub use crate::{
    huff_structs::{
        HuffLeaf, 
        HuffBranch, 
        HuffTree, 
        HuffCode,
        ByteFreqs,
    },
    file::{
        compress,
        threaded_compress,
        decompress,
    },
};

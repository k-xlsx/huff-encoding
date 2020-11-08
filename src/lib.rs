/// Module containing methods to compress/decompress bytes
pub mod file;
/// Module containing structs used to build a *Huffman Tree*
pub mod huff_structs;

mod utils;

// TODO: make more extensive docs

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

pub mod file;

mod utils;
mod huff_structs;


pub use crate::huff_structs::{
    HuffLeaf, 
    HuffBranch, 
    HuffTree, 
    HuffCode,
    ByteFreqs,
};

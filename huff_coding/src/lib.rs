//! TODO: docs

// TODO: compression/decompression
// TODO: tests

/// Module containing structs used to build a *Huffman Tree*
pub mod tree;
/// Module containing the `Weights` trait and the `byte_weights` module
pub mod weights;
/// `huff_coding` symbol export
pub mod prelude{
    pub use super::tree::{
        HuffLetter,
        HuffLetterAsBytes,
        HuffBranch,
        HuffLeaf, 
        HuffTree,
        FromBinError,
    };
    pub use super::weights::{
        Weights,
        byte_weights::ByteWeights
    };
}
mod utils;

// `bitvec` re-export
pub use bitvec;

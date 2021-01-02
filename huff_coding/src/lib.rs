//! TODO: docs

// TODO: compression/decompression
// TODO: tests

/// Module containing structs used to build a *Huffman Tree*
pub mod tree;
/// Module containing the `Freq` trait and the `byte_freqs` module
pub mod freqs;
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
    pub use super::freqs::{
        Freq,
        byte_freqs::ByteFreqs
    };
}
mod utils;

// `bitvec` re-export
pub use bitvec;



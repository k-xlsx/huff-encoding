/// Module containing structs used to build a *Huffman Tree*
pub mod tree;
pub mod freqs;

mod utils;

// TODO: make extensive docs
// TODO: compression/decompression
// TODO: tests

pub mod prelude{
    pub use super::tree::{
        HuffLetter,
        HuffLetterBitStore,
        HuffBranch,
        HuffLeaf, 
        HuffTree,
    };
    pub use super::freqs::{
        Freq,
        byte_freqs::ByteFreqs
    };
}

pub use freqs::{
    byte_freqs::{self, ByteFreqs}
};

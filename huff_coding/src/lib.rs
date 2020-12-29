/// Module containing structs used to build a *Huffman Tree*
pub mod tree;
/// Module containing the ```Freq``` trait and the ```byte_freqs``` module
pub mod freqs;

mod utils;

// TODO: compression/decompression
// TODO: tests
// TODO: rename the 'tree' folder

/// ```huff_coding``` symbol export
pub mod prelude{
    pub use super::tree::{
        HuffLetter,
        HuffLetterAsBytes,
        HuffCode,
        HuffTreeBin,
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

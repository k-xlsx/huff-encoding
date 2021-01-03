//! An implementation of the `Huffman coding` algorithm, enabling
//! one to create a `Huffman tree` with any alphabet they choose.
//! 
//! It mainly revolves around the `HuffTree` struct, which provides a way to
//! generate `Huffman codes` for any collection of types implementing
//! the `HuffLetter` struct, where for every letter there is a corresponding weight
//! (To ensure this, the `Weights` trait must be implemented on the provided collection).
//! If the provided letters also implement the `HuffLetterAsBytes` trait, the `HuffTree`
//! can be easily read or returned in binary form. See `HuffTree`'s documentation for examples
//! and more extensive explanation.
//! 
//! Every binary representation in the crate is made thanks to the `bitvec` crate which
//! I've re-exported for convenience.
//! 
//! I am still working on example compression/decompression functions
//! using my implementation of this algorithm.

// TODO: compression/decompression
// TODO: tests
// TODO: links in docs

/// Struct representing a Huffman Tree
pub mod tree;
/// Struct representing a branch in the `HuffTree` struct. 
pub mod branch;
/// Struct representing a `HuffBranch`'s data.
pub mod leaf;
/// Traits signyfing that a type can be stored in a `HuffTree` as a letter
pub mod letter;
/// Trait signifying that a struct stores the weights of a type `L`, so that
/// for any stored `L` there is a corresponding `usize`(weight), and 
/// an implementation of it over bytes. 
pub mod weights;
/// `huff_coding` prelude
///
/// This collects the general public API into a single spot for inclusion, as
/// `use huff_coding::prelude::*;`, without polluting the root namespace of the crate
pub mod prelude;

mod utils;
mod branch_heap;

// `bitvec` re-export
pub use bitvec;

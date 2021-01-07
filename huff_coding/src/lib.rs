//! An implementation of the [Huffman coding algorithm][huff_wiki], enabling
//! one to create a [Huffman tree][huff_wiki_expl] with any alphabet they choose.
//! 
//! It mainly revolves around the [`HuffTree`][tree] struct, which provides a way to
//! generate [Huffman prefix codes][huff_wiki_codes] for any collection of types implementing
//! the [`HuffLetter`][letter] trait, where for every letter there is a corresponding weight
//! (To ensure this, the [`Weights`][weights] trait must be implemented on the provided collection).
//! If the provided letters also implement the [`HuffLetterAsBytes`][letter_bytes] trait, 
//! the [`HuffTree`][tree] can be easily read or returned in binary form. 
//! See [`HuffTree`][tree] documentation for examples and more extensive explanation.
//! 
//! Every binary representation in the crate is made thanks to the [`bitvec`][bitvec] crate which
//! I've re-exported for convenience.
//! 
//! I am still working on example compression/decompression functions
//! using my implementation of this algorithm.
//! 
//! [tree]:tree::HuffTree
//! [letter]:tree::letter::HuffLetter
//! [letter_bytes]:tree::letter::HuffLetterAsBytes
//! [weights]:weights::Weights
//! [huff_wiki]:https://en.wikipedia.org/wiki/Huffman_coding
//! [huff_wiki_expl]:https://en.wikipedia.org/wiki/Huffman_coding#Basic_technique
//! [huff_wiki_codes]:https://en.wikipedia.org/wiki/Prefix_code

// TODO: decompression
// TODO: serde


pub mod tree;
/// Trait signifying that a struct stores the weights of a type `L`, so that
/// for any stored `L` there is a corresponding `usize`(weight), and 
/// an implementation of it over bytes. 
pub mod weights;
/// Example compression/decompression functions using the [`HuffTree`][crate::tree::HuffTree] struct
// TODO: find a better name
pub mod cmpr;
/// `huff_coding` prelude.
///
/// This collects the general public API into a single spot for inclusion, as
/// `use huff_coding::prelude::*;`, without polluting the root namespace of the crate.
pub mod prelude;

mod utils;


// `bitvec` re-export
pub use bitvec;

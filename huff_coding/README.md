# **huff_coding**

[![Crate][crate_img]][crate]
[![Documentation][docs_img]][docs]

[![License][license_img]][license_file]

An implementation of the [Huffman coding algorithm][huff_wiki], enabling
one to create a Huffman tree with any alphabet they choose.

It mainly revolves around the [`HuffTree`][tree] struct, which provides a way to generate Huffman [prefix codes][huff_wiki_codes] for any collection of types implementing the [`HuffLetter`][letter] trait, where  for every letter there is a corresponding weight (To ensure this, the [`Weights`][weights] trait must be implemented on the provided collection).
If the provided letters also implement the [`HuffLetterAsBytes`][letter_bytes] trait, the tree can be easily read or returned in binary form.

## Examples

```rust
use huff_coding::{

prelude::*,

bitvec::prelude::*,

};

  

// every primitive type (except floats) implements HuffLetter
let bytes = [0xff, 0xff, 0xff, 0xaa, 0xaa, 0xcc];
let chars = ['a', 'a', 'a', 'b', 'b', 'c'];
let ints = [-32, 123, -32, -32, 75, 123];


// ------ building weights structs ------

// building weights with the ByteWeights struct
let byte_weights =  ByteWeights::from_bytes(&bytes);
// building weights in the form of a HashMap
let char_weights =  build_weights_map(&chars);
let int_weights =  build_weights_map(&ints);


// ------ initializing HuffTrees ------

let tree_bytes =  HuffTree::from_weights(byte_weights);
let tree_chars =  HuffTree::from_weights(char_weights);
let tree_ints =  HuffTree::from_weights(int_weights);


// ------ reading codes from a tree ------
let char_codes = tree_chars.read_codes();
assert_eq!(
    char_codes.get(&'a').unwrap(),
    &bitvec![Msb0, u8; 0]
);
assert_eq!(
    char_codes.get(&'b').unwrap(),
    &bitvec![Msb0, u8; 1, 1]
);
assert_eq!(
    char_codes.get(&'c').unwrap(),
    &bitvec![Msb0, u8; 1, 0]
);


// ------ HuffTree in binary ------

// every integer implements HuffLetterAsBytes
let tree_bytes_bin = tree_bytes.as_bin();
assert_eq!(tree_bytes_bin.to_string(), "[10111111, 11101100, 11000101, 01010]");
// reading a HuffTree from a binary representation
let tree_bytes_from_bin =  HuffTree::<u8>::try_from_bin(tree_bytes_bin).unwrap();
assert_eq!(tree_bytes.read_codes(), tree_bytes_from_bin.read_codes());
```

Included are also example compression/decompression functions using my implementation of this algorithm.

```rust
use huff_coding::prelude::*;



let bytes =  b"abbccc";
let comp_data =  compress(bytes);
let decomp_bytes =  decompress(&comp_data);

assert_eq!(bytes.to_vec(), decomp_bytes);
```

Every binary representation in the crate is made thanks to the [`bitvec`][bitvec] crate which I've re-exported for convenience.

[license_file]:https://github.com/kxlsx/huffman-coding-rs/blob/master/LICENSE
[license_img]: https://img.shields.io/crates/l/huff_coding.svg
[crate]:https://crates.io/crates/huff_coding
[crate_img]:https://img.shields.io/crates/v/huff_coding.svg?logo=rust
[docs]:https://docs.rs/huff_coding/0.0.1/huff_coding/
[docs_img]:https://docs.rs/huff_coding/badge.svg

[huff_wiki]:https://en.wikipedia.org/wiki/Huffman_coding
[huff_wiki_codes]:https://en.wikipedia.org/wiki/Prefix_code

[tree]:https://github.com/kxlsx/huffman-coding-rs/blob/master/huff_coding/src/tree/tree_inner.rs#L17
[letter]:https://github.com/kxlsx/huffman-coding-rs/blob/master/huff_coding/src/tree/letter.rs#L10
[letter_bytes]:https://github.com/kxlsx/huffman-coding-rs/blob/master/huff_coding/src/tree/letter.rs#L16
[weights]:https://github.com/kxlsx/huffman-coding-rs/blob/master/huff_coding/src/weights.rs#L19
[bitvec]:https://github.com/bitvecto-rs/bitvec

use super::{
    prelude::{
        HuffTree,
        HuffLetter,
        HuffLetterAsBytes,
        build_weights_map,
    },
    utils::calc_padding_bits,
};

use std::{
    fmt,
    marker::PhantomData,
};



/// Data representing a slice of letters (types implementing [`HuffLetter`][letter]) 
/// compressed into bytes by the [`compress`][compress] or [`compress_with_tree`][compress_with_tree] 
/// function.
/// 
/// It stores:
/// * `L` -> generic type of the compressed letters
/// * [`comp_bytes`](#method.comp_bytes) -> representing the compressed slice
/// * [`padding_bits`](#method.padding_bits) -> the number of bits used for padding in the comp_bytes
/// * [`huff_tree`](#method.huff_tree) -> the [`HuffTree`][tree] used to compress the slice
/// 
/// If the letter type also implements [`HuffLetterAsBytes`][letter_bytes], the compressed
/// data can be easily represented as bytes (see the [`to_bytes`](#method.to_bytes) method's 
/// docs for more information).
/// 
/// [tree]:crate::tree::HuffTree
/// [letter]:crate::tree::letter::HuffLetter
/// [letter_bytes]:crate::tree::letter::HuffLetterAsBytes
#[derive(Debug, Clone)]
pub struct CompressedData<L: HuffLetter>{
    comp_bytes: Vec<u8>,
    padding_bits: u8,
    huff_tree: HuffTree<L>,
    _typebind: PhantomData<L>
}

impl<L: HuffLetter> CompressedData<L>{
    /// Initialize a new instance of [`CompressedData`][CompressedData] with the provided
    /// compressed bytes, padding bits and [`HuffTree`][crate::tree::HuffTree].
    pub fn new(comp_bytes: Vec<u8>, padding_bits: u8, huff_tree: HuffTree<L>) -> Self{
        Self{
            comp_bytes,
            padding_bits,
            huff_tree,
            _typebind: PhantomData::default(),
        }
    }

    /// Return a reference to the stored slice compressed into bytes
    pub fn comp_bytes(&self) -> &[u8]{
        &self.comp_bytes
    }

    /// Return the number of bits used for padding in the compressed slice
    pub fn padding_bits(&self) -> u8{
        self.padding_bits
    }

    /// Return a reference to the [`HuffTree`][crate::tree::HuffTree] used to compress the slice
    pub fn huff_tree(&self) -> &HuffTree<L>{
        &self.huff_tree
    }
}

impl<L: HuffLetterAsBytes> CompressedData<L>{
    /// 
    pub fn to_bytes(self) -> Vec<u8>{
        // get tree in binary, 
        // calculate its padding bits when converted to bytes
        // calculate its lenght in bytes
        let tree_bin = self.huff_tree().as_bin();
        let tree_bin_padding_bits = calc_padding_bits(tree_bin.len());
        let tree_bytes_len = (tree_bin.len() as u32 + tree_bin_padding_bits as u32) / 8;

        let mut bytes = Vec::new();
        // push an empty byte, later to be filled by the padding bit nums
        bytes.push((tree_bin_padding_bits << 4) + self.padding_bits());
        // push the length  of the tree_bin (4 byte num)
        bytes.extend(
            tree_bytes_len.to_be_bytes().iter()
        );
        // next push the tree in binary
        bytes.append(&mut tree_bin.into_vec());
        
        bytes.extend(self.comp_bytes());
 
        bytes
    }
}

/// Error encountered while compressing, meaning that
/// a byte hasn't been found in the provided codes.
/// 
/// Returned by [`compress_with_tree`][compress_with_tree] 
#[derive(Debug, Clone)]
pub struct CompressError<L: HuffLetter>{
    message: &'static str,
    missing_letter: L,
}

impl<L: HuffLetter> fmt::Display for CompressError<L>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?})", self.message, self.missing_letter)
    }
}

impl<L: HuffLetter> std::error::Error for CompressError<L>{}

impl<L: HuffLetter> CompressError<L>{
    pub fn new(message: &'static str, missing_letter: L) -> Self{
        Self{
            message,
            missing_letter,
        }
    }

    pub fn message(&self) -> &str{
        self.message
    }

    pub fn missing_letter(&self) -> &L{
        &self.missing_letter
    }
}


/// Compress the provided slice of letters (types implementing [`HuffLetter`][letter]), using binary
/// codes generated with a [`HuffTree`][tree] struct, into a byte slice (returned with additional data
/// in the form of [`CompressedData`][CompressedData]).
/// 
/// The letters are counted into a [`Weights`][weights] collection to create a [`HuffTree`][tree] using
/// the [`build_weights_map`][weights_map] function, which can be optimized a lot for certain letter types.
/// Because of this fact, it's generally faster to use the [`compress_with_tree`][compress_with_tree] function,
/// providing a [`HuffTree`][tree] built with our own [`Weights`][weights] collection (an example of such collection
/// is implemented in the crate on `u8` in the form of [`ByteWeights`][byte_weights]).
/// 
/// The returned [`CompressedData`][CompressedData] can be decompressed into the original letter slice with
/// the [`decompress`][decompress] function.
/// 
/// # How it works
/// ---
/// It just reads every letter's code in the created [`HuffTree`][tree] and inserts them into
/// a [`Vec<u8>`][Vec]. The codes themselves are mostly not a multiple of 8 bits long, so some
/// of them can be used as padding in the last byte. The padding information, as well as
/// the tree used to compress the slice are included in the returned [`CompressedData`][CompressedData].
/// 
/// # Example
/// ---
/// ```
/// use huff_coding::prelude::{
///     compress,
///     decompress
/// };
/// 
/// let bytes = b"abbccc";
/// let nums = &[-97, -98, -98, -99, -99, -99];
/// let chars = &['a', 'b', 'b', 'c', 'c', 'c'];
/// let strs = &["ay", "bee", "bee", "cee", "cee", "cee"];
/// 
/// let comp_bytes = compress(bytes);
/// let comp_nums = compress(nums);
/// let comp_chars = compress(chars);
/// let comp_strs = compress(strs);
/// 
/// assert_eq!(bytes.to_vec(), decompress(&comp_bytes));
/// assert_eq!(nums.to_vec(), decompress(&comp_nums));
/// assert_eq!(chars.to_vec(), decompress(&comp_chars));
/// assert_eq!(strs.to_vec(), decompress(&comp_strs));
/// ```
/// 
/// [tree]:crate::tree::HuffTree
/// [letter]:crate::tree::letter::HuffLetter
/// [weights]:crate::weights::Weights
/// [weights_map]:crate::weights::build_weights_map
/// [byte_weights]:crate::weights::ByteWeights
pub fn compress<L: HuffLetter>(letters: &[L]) -> CompressedData<L>{
    let huff_tree = HuffTree::from_weights(build_weights_map(letters));
    compress_with_tree(letters, huff_tree).unwrap()
}

/// Compress the provided slice of letters (types implementing [`HuffLetter`][letter]), using binary
/// codes generated with the provided [`HuffTree`][tree] struct, into a byte slice (returned with additional 
/// data in the form of [`CompressedData`][CompressedData]).
/// 
/// The returned [`CompressedData`][CompressedData] can be decompressed into the original letter slice with
/// the [`decompress`][decompress] function.
/// 
/// # How it works
/// ---
/// It just reads every letter's code in the provided [`HuffTree`][tree] and inserts them into
/// a [`Vec<u8>`][Vec]. The codes themselves are mostly not a multiple of 8 bits long, so some
/// of them can be used as padding in the last byte. The padding information, as well as
/// the tree used to compress the slice are included in the returned [`CompressedData`][CompressedData].
/// 
/// # Example
/// ---
/// ```
/// use huff_coding::prelude::{
///     compress_with_tree,
///     decompress,
///     HuffTree,
///     ByteWeights,
/// };
/// 
/// let bytes = b"abbccc";
/// 
/// let tree = HuffTree::from_weights(
///     ByteWeights::from_bytes(bytes)
/// );
/// let comp_bytes = compress_with_tree(bytes, tree).unwrap();
/// 
/// assert_eq!(bytes.to_vec(), decompress(&comp_bytes));
/// ```
/// 
/// # Errors
/// ---
/// When the provided tree does not contain a code 
/// for a letter in the provided slice:
/// ```should_panic
/// use huff_coding::prelude::{
///     compress_with_tree,
///     HuffTree,
///     ByteWeights,
/// };
/// 
/// let bytes = b"abbccc";
/// let other_bytes = b"abb";
/// 
/// let tree = HuffTree::from_weights(
///     ByteWeights::from_bytes(other_bytes)
/// );
/// 
/// let comp_bytes = compress_with_tree(bytes, tree)
///     .expect("this will panic, letter b'c' not found in codes");
/// ```
/// 
/// [tree]:crate::tree::HuffTree
/// [letter]:crate::tree::letter::HuffLetter
pub fn compress_with_tree<L: HuffLetter>(letters: &[L], huff_tree: HuffTree<L>) -> Result<CompressedData<L>, CompressError<L>>{
    let mut compressed_letters = Vec::with_capacity(letters.len());
    let codes = huff_tree.read_codes();
    let mut current_byte = 0b0000_0000;
    let mut i = 7;
    for letter in letters{
        // return Err if there's no code
        let code = 
            if let Some(code) = codes.get(letter){Ok(code)}
            else{
                Err(CompressError::new(
                    "letter not found in codes", 
                    letter.clone()))
            }?;
        for bit in code{
            // set bit on current byte
            current_byte |= (*bit as u8) << i;
            // if filled current_byte
            if i == 0{
                compressed_letters.push(current_byte);
                current_byte = 0b0000_0000;
                i = 7;
            }
            else{i -= 1};
        }
    }
    // calculate the compressed_letters' padding bits
    let padding_bits = if i == 7{0} else{i + 1};
    if padding_bits != 0{compressed_letters.push(current_byte);}


    Ok(CompressedData::new(compressed_letters, padding_bits, huff_tree))
}

/// Decompress the provided [`CompressedData<L>`][CompressedData] into a [`Vec<L>`][Vec].
/// 
/// # How it works
/// ---
/// 1. Start at the root branch of the tree
/// 2. Go bit by bit through the provided [`CompressedData`'s][CompressedData] comp_bytes
/// 3. Every time a 0 is found, go to the left branch, and 
/// every 1 means going to the right branch
/// 4. When it finally a letter branch is found, it push the letter into
/// the vec, and return to the root branch.
/// 
/// # Example
/// ---
/// ```
/// use huff_coding::prelude::{
///     compress,
///     decompress
/// };
/// 
/// let bytes = b"deefff";
/// let nums = &[-100, -101, -101, -102, -102, -102];
/// let chars = &['d', 'e', 'e', 'f', 'f', 'f'];
/// let strs = &["dee", "e", "e", "ef", "ef", "ef"];
/// 
/// let comp_bytes = compress(bytes);
/// let comp_nums = compress(nums);
/// let comp_chars = compress(chars);
/// let comp_strs = compress(strs);
/// 
/// assert_eq!(bytes.to_vec(), decompress(&comp_bytes));
/// assert_eq!(nums.to_vec(), decompress(&comp_nums));
/// assert_eq!(chars.to_vec(), decompress(&comp_chars));
/// assert_eq!(strs.to_vec(), decompress(&comp_strs));
/// ```
pub fn decompress<L: HuffLetter>(comp_data: &CompressedData<L>) -> Vec<L>{
    let bytes = comp_data.comp_bytes();
    let tree = comp_data.huff_tree();

    let mut decompressed_letters = Vec::new();
    let mut current_branch = tree.root();
    macro_rules! read_codes_in_byte {
        ($byte: expr;[$bitrange:expr]) => {
            for i in $bitrange{
                if current_branch.has_children(){
                    match ($byte >> (7 - i)) & 1 == 1{
                        true =>{
                            current_branch = current_branch.right_child().unwrap();
                        }
                        false =>{
                            current_branch = current_branch.left_child().unwrap();
                        }
                    }
                }
                if !current_branch.has_children(){
                    decompressed_letters.push(current_branch.leaf().letter().unwrap().clone());
                    current_branch = tree.root();
                }
            }
        };
    }
    for byte in &bytes[..bytes.len() - 1]{
       read_codes_in_byte!(byte;[0..8]);
    }
    read_codes_in_byte!(bytes[bytes.len() - 1];[0..8 - comp_data.padding_bits()]);

    decompressed_letters
}

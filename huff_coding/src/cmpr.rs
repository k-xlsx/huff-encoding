use super::{
    prelude::{
        HuffTree,
        ByteWeights,
    },
    bitvec::prelude::{
        BitVec, 
        Msb0
    },
    utils::calc_padding_bits,
};

use std::{
    fmt,
    collections::HashMap,
};



/// Error encountered while compressing, meaning that
/// a byte hasn't been found in the provided codes.
/// 
/// Returned by [`compress_with_tree`][compress_with_tree] and 
/// [`get_compressed_bytes`][get_compressed_bytes]
#[derive(Debug, Clone)]
pub struct CompressError{
    message: &'static str,
    missing_byte: u8,
}

impl fmt::Display for CompressError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:x})", self.message, self.missing_byte)
    }
}

impl std::error::Error for CompressError{}

impl CompressError{
    pub fn new(message: &'static str, missing_byte: u8) -> Self{
        Self{
            message,
            missing_byte,
        }
    }

    pub fn message(&self) -> &str{
        self.message
    }

    pub fn missing_byte(&self) -> u8{
        self.missing_byte
    }
}


/// Example compression function using a [HuffTree's codes][codes]
/// to replace every byte in the given slice.
/// 
/// It's reasonably fast for smaller files, but starts being exponentially 
/// slower around 5GB.
/// Returns a [`Vec<u8>`][Vec] containing the compressed bytes, as well as the
/// data used to decompress them.
/// 
/// # Encoding scheme
/// ---
/// The returned bytes store, in order:
/// 1. A byte containing the number of bits used for padding:
///  * first 4 bits store the [HuffTree's][tree] padding bits
///  * the remaining bits store the compressed data's padding bits
/// 2. 4 byte number representing the length (in bytes) of the stored [`HuffTree`][tree]
/// 3. A [`HuffTree`][tree], used to compress the file, 
/// represented in binary (see [`HuffTree::try_from_bin`][from_bin])
/// 4. The actual compressed data
/// 
/// # Example
/// –––
/// Here's a manual deconstruction of the compressed
/// data:
/// ```
/// use huff_coding::{
///     prelude::{
///         compress,
///         HuffTree,
///     },
///     bitvec::prelude::*,
/// };
/// use std::{
///     convert::TryInto,
///     collections::HashMap,
/// };
/// 
/// // get compressed data
/// let compressed = compress(b"abbccc");
/// 
/// // first byte stores the padding bits, 
/// // in this case:
/// // * 3 padding bits used for the tree
/// // * 7 padding bits used for the data
/// assert_eq!(
///     compressed[0], 
///     0x37
/// );
/// 
/// // the next 4 bytes store the tree's length, 
/// // in this case: 4 bytes
/// assert_eq!(
///     u32::from_be_bytes(compressed[1..5].try_into().unwrap()), 
///     4
/// );
/// 
/// // next 4 bytes (as read from the length) store the tree,
/// // in this case it stores the following codes:
/// // b'a' - 10
/// // b'b' - 11
/// // b'c' - 0
/// let codes = 
///     HuffTree::<u8>::try_from_bin({
///         // get the next 4 bytes from compressed and remove the specified 3 padding bits
///         let mut b = BitVec::from_vec(compressed[5..9].to_vec());
///         b.drain(29..);
///         b
///     })
///     .expect("this passes just fine")
///     .read_codes();
/// 
/// let mut cmp_codes = HashMap::new();
/// cmp_codes.insert(b'a', bitvec![Msb0, u8; 1, 0]);
/// cmp_codes.insert(b'b', bitvec![Msb0, u8; 1, 1]);
/// cmp_codes.insert(b'c', bitvec![Msb0, u8; 0]);
/// 
/// assert_eq!(codes, cmp_codes);
/// 
/// // the last bytes (containing the compressed data) are:
/// assert_eq!(compressed[9], 0b10111100);
/// assert_eq!(compressed[10], 0b00000000);
/// ```
/// now we could easily read the actual data:
/// 1. 10111100:
///  * 10 -> `b'a'`
///  * 11 -> `b'b'`
///  * 11 -> `b'b'`
///  * 0  -> `b'c'`
///  * 0  -> `b'c'`
/// 2. 00000000:
///  * 0  -> `b'c'`
///  * the remaining 7 bits are used for padding.
/// 
/// And thus we succesfully read the bytes `b"abbccc"`!
/// 
/// [tree]:crate::tree::HuffTree
/// [codes]:../tree/struct.HuffTree.html#method.read_codes
/// [from_bin]:../tree/struct.HuffTree.html#method.try_from_bin

pub fn compress(bytes: &[u8]) -> Vec<u8>{
    let huff_tree = HuffTree::from_weights(ByteWeights::from_bytes(&bytes));
    compress_with_tree(bytes, &huff_tree).expect(
        "invalid tree made from the same bytes - something has gone very wrong"
    )
}

/// Similar to [`compress`][compress], 
/// but with the ability provide your own [`HuffTree`][tree]
/// 
/// See [`compress`'][compress] documentation for a more general explanation.
/// 
/// # Errors
/// ---
/// If the tree's codes are missing a byte found in the given slice 
/// (basically the tree's invalid):
/// ```should_panic
/// use huff_coding::prelude::{
///     HuffTree,
///     ByteWeights,
///     compress_with_tree,
/// };
/// 
/// let tree = HuffTree::from_weights(ByteWeights::from_bytes(b"abbccc"));
/// 
/// let compressed_bytes = compress_with_tree(b"deefff", &tree)
///     .expect("this will return a CompressError (missing byte 0x64)");
/// ```
/// 
/// [tree]: crate::tree::HuffTree
pub fn compress_with_tree(bytes: &[u8], huff_tree: &HuffTree<u8>) -> Result<Vec<u8>, CompressError>{
    // get tree in binary, 
    // calculate its padding bits when converted to bytes
    // calculate its lenght in bytes
    let tree_bin = huff_tree.as_bin();
    let tree_bin_padding_bits = calc_padding_bits(tree_bin.len());
    let tree_bytes_len = (tree_bin.len() as u32 + tree_bin_padding_bits as u32) / 8;

    let mut compressed = Vec::with_capacity(bytes.len());
    // push an empty byte, later to be filled by the padding bit nums
    compressed.push(0);
    // push the length of the tree_bin (4 byte num)
    compressed.extend(
        tree_bytes_len.to_be_bytes().iter()
    );
    // next push the tree in binary 
    compressed.append(&mut tree_bin.into_vec());

    // convert bytes to compressed bytes and push them into compressed
    let padding_bits = push_compressed_bytes(&mut compressed, bytes, &huff_tree.read_codes())?;
    // set the padding bits of the tree_bin and compressed bytes at the beginning
    // (first 4 bits -> tree, remaining 4 -> compressed bytes)
    compressed[0] = (tree_bin_padding_bits << 4) + padding_bits;

    Ok(compressed)
}

/// Returns **ONLY** the bytes compressed using the provided codes and the number of bits used for padding. 
/// 
/// The returned [`Vec<u8>`][Vec] **DOES NOT** contain the data needed to decompress the bytes
/// (contrary to [`compress`][compress] and [`compress_with_tree`][compress_with_tree]).
/// 
/// # Example
/// ---
/// ```
/// use huff_coding::prelude::{
///     HuffTree,
///     ByteWeights,
///     get_compressed_bytes,
/// };
/// 
/// let bytes = b"abbccc";
/// 
/// let codes = HuffTree::from_weights(ByteWeights::from_bytes(bytes)).read_codes();
/// 
/// let (compressed_bytes, padding_bits) = get_compressed_bytes(bytes, &codes).unwrap();
/// 
/// // every b'a' has been replaced by 10, every b'b' by 11, and every b'c',
/// // so the resulting compressed bytes look like this:
/// assert_eq!(
///     compressed_bytes, 
///     vec![0b10111100, 0b00000000]
/// );
/// // with the last 7 bits being used for padding
/// assert_eq!(padding_bits, 7);
/// ```
/// 
/// # Errors
/// ---
/// If the codes are missing a byte found in the given slice 
/// ```should_panic
/// use huff_coding::prelude::{
///     HuffTree,
///     ByteWeights,
///     get_compressed_bytes,
/// };
/// 
/// let codes = HuffTree::from_weights(ByteWeights::from_bytes(b"abbccc")).read_codes();
/// 
/// let (compressed_bytes, padding_bits) = get_compressed_bytes(b"deefff", &codes)
///     .expect("this will return a CompressError (missing byte 0x64)");
/// ```
pub fn get_compressed_bytes(bytes: &[u8], codes: &HashMap<u8, BitVec<Msb0, u8>>) -> Result<(Vec<u8>, u8), CompressError>{
    let mut compressed_bytes = Vec::with_capacity(bytes.len());
    // convert bytes to compressed bytes and push them into compressed
    let padding_bits = push_compressed_bytes(&mut compressed_bytes, bytes, codes)?;
    Ok((compressed_bytes, padding_bits))
}

/// Compress given bytes using provided codes, pushing them 
/// into the given compressed_bytes vec. Afterwards return the
/// number of bits used for padding
fn push_compressed_bytes(compressed_bytes: &mut Vec<u8>, bytes: &[u8], codes: &HashMap<u8, BitVec<Msb0, u8>>) -> Result<u8, CompressError>{
    // read every byte's code and push it into compressed_bytes
    // basically just tries to fill a byte with given codes' bits,
    // and then push it
    let mut current_byte = 0b0000_0000;
    let mut i = 7;
    for byte in bytes{
        // return Err if there's no code
        let code = 
            if let Some(code) = codes.get(byte){Ok(code)}
            else{
                Err(CompressError::new(
                    "byte not found in codes", 
                    *byte))
            }?;
        for bit in code{
            // set bit on current byte
            current_byte |= (*bit as u8) << i;
            // if filled current_byte
            if i == 0{
                compressed_bytes.push(current_byte);
                current_byte = 0b0000_0000;
                i = 7;
            }
            else{i -= 1};
        }
    }
    // calculate the compressed bytes' padding bits
    let padding_bits = if i == 7{0} else{i + 1};
    if padding_bits != 0{compressed_bytes.push(current_byte);}

    // return the compressed bytes' padding bits
    Ok(padding_bits)
}

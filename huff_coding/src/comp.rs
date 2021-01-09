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
    convert::TryInto,
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

/// Error encountered while decompressing.
/// 
/// Returned by [`decompress`][decompress]
#[derive(Debug, Clone)]
pub struct DecompressError{
    message: &'static str,
}

impl fmt::Display for DecompressError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DecompressError{}

impl DecompressError{
    pub fn new(message: &'static str) -> Self{
        Self{
            message,
        }
    }

    pub fn message(&self) -> &str{
        self.message
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
/// Here's a 'manual' deconstruction of the compressed
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
/// # Panics
/// ---
/// When providing an empty byte slice:
/// ```should_panic
/// use huff_coding::prelude::compress;
/// 
/// // Panics at trying to create a HuffTree with no letters
/// compress(b"");
/// ```
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

/// Returns **ONLY** the bytes compressed using the provided tree's codes and the number of bits used for padding.
/// `(Vec<u8>, u8)`
/// 
/// The returned [`Vec<u8>`][Vec] **DOES NOT** contain the data needed to decompress the bytes
/// (contrary to [`compress`][compress] and [`compress_with_tree`][compress_with_tree]).
/// It's the opposite of [`get_decompressed_bytes`][get_decompressed_bytes]
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
/// let tree = HuffTree::from_weights(ByteWeights::from_bytes(bytes));
/// 
/// let (compressed_bytes, padding_bits) = get_compressed_bytes(bytes, &tree).unwrap();
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
/// let tree = HuffTree::from_weights(ByteWeights::from_bytes(b"abbccc"));
/// 
/// let (compressed_bytes, padding_bits) = get_compressed_bytes(b"deefff", &tree)
///     .expect("this will return a CompressError (missing byte 0x64)");
/// ```
pub fn get_compressed_bytes(bytes: &[u8], huff_tree: &HuffTree<u8>) -> Result<(Vec<u8>, u8), CompressError>{
    let mut compressed_bytes = Vec::with_capacity(bytes.len());
    // convert bytes to compressed bytes and push them into compressed
    let padding_bits = push_compressed_bytes(&mut compressed_bytes, bytes, &huff_tree.read_codes())?;
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

/// Example decompression function bytes compressed with [`compress`][compress]
/// 
/// # Decoding scheme
/// ---
/// 1. Read padding information from the first byte:
///  * first 4 bits represent the tree's padding bits
///  * the remaining bits represent the compressed data's
/// padding bits
/// 2. Read the next 4 bytes as a `u32` representing the encoded
/// tree's length in bytes
/// 3. Build a [`HuffTree`][tree] from the next `tree_len * 8 - tree_padding_bits` bits
/// 4. Decompress the remaining contents with the built tree, going bit by bit:
///  * every encountered 0 means going to the left child
///  * every 1 means the right child
///  * upon finding a letter branch, store the letter and reset the current branch
/// to tree's root
/// 
/// # Example
/// ---
/// Basic use:
/// ```
/// use huff_coding::prelude::{
///     compress,
///     decompress,
/// };
/// 
/// let bytes = b"mirek";
/// let comp = compress(bytes);
/// let decomp = decompress(&comp).unwrap();
/// 
/// assert_eq!(bytes.to_vec(), decomp);
/// ```
/// 
/// # Errors
/// ---
/// A provided slice must contain at least `5 + tree_len + 1` bytes:
/// * 5 bytes reserved for tree_len and padding
/// * a [`HuffTree`][tree] encoded in binary of size tree_len bytes
/// * at least 1 byte of the actual compressed data
/// 
/// [tree]:crate::tree::HuffTree
/// [from_bin]:../tree/struct.HuffTree.html#method.try_from_bin
pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, DecompressError>{
    /// Returns DecompressError with the given message 
    /// if the index is out of bounds of bytes
    macro_rules! bytes_try_get {
        [$index:expr; $message:expr] => {
            if let Some(subslice) = bytes.get($index){
                Ok(subslice)
            }
            else{
                Err(DecompressError::new($message))
            }
        };
    }

    // get padding data
    let padding_bits = bytes_try_get![0; "slice is empty"]?;
    let tree_padding_bits =  padding_bits >> 4;
    let data_padding_bits = padding_bits & 0b0000_1111;

    // read 4 bytes of tree length
    let tree_len = u32::from_be_bytes(
        bytes_try_get![1..5; "slice too short to read tree length"]?
        .try_into()
        .unwrap()
    ) as usize;
    assert!(tree_len >= 2, "stored tree length must be at least 2");

    // read the tree
    let tree_from_bin_result = 
        HuffTree::<u8>::try_from_bin({
            let mut b = BitVec::from_vec(
                bytes_try_get![5..5 + tree_len; "slice too short to read tree"]?
                .to_vec()
            );
            for _ in 0..tree_padding_bits{b.pop();}
            b
        });
    let tree = 
        if let Ok(tree) = tree_from_bin_result{
            tree
        }
        else{
            return Err(DecompressError::new(
                "invalid tree in slice"
            ))
        };
    
    Ok(get_decompressed_bytes(
        bytes_try_get![5 + tree_len..; "slice does not contain compressed data"]?, 
        data_padding_bits, 
        &tree
    ))
}

/// Decompress the given bytes in accordance with the given 
/// [`HuffTree`][crate::tree::HuffTree] and padding_bits
/// 
/// This function *ONLY* decompresses the given data, so be sure not to provide it the
/// raw output of [`compress`][compress], as it also stores additional information encoded in the 
/// first bytes (see [`compress`][compress] or [`decompress`][decompress] docs for elaboration).
/// It's the opposite of [`get_compressed_bytes`][get_compressed_bytes]
/// 
/// # Example
/// ---
/// ```
/// use huff_coding::prelude::{
///     HuffTree,
///     ByteWeights,
///     get_compressed_bytes,
///     get_decompressed_bytes,
/// };
/// 
/// let bytes = b"abbccc";
/// let tree = HuffTree::from_weights(ByteWeights::from_bytes(bytes));
/// let (compressed_bytes, padding_bits) = get_compressed_bytes(bytes, &tree).unwrap();
/// 
/// let decompressed_bytes = get_decompressed_bytes(&compressed_bytes, padding_bits, &tree);
/// 
/// assert_eq!(bytes.to_vec(), decompressed_bytes);
/// ```
/// 
/// # Panics & Errors
/// ---
/// This function always tries to decode the provided bytes with the given tree
/// and should not panic or return an error. The worst that could happen, when providing
/// incompatible tree & bytes (ie. the tree's not built from the bytes' weights), 
/// is that the output may be empty or jumbled a little.
pub fn get_decompressed_bytes(bytes: &[u8], padding_bits: u8, huff_tree: &HuffTree<u8>) -> Vec<u8>{
    let mut decompressed_bytes = Vec::new();
    let mut current_branch = huff_tree.root();
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
                    decompressed_bytes.push(*current_branch.leaf().letter().unwrap());
                    current_branch = huff_tree.root();
                }
            }
        };
    }
    for byte in &bytes[..bytes.len() - 1]{
       read_codes_in_byte!(byte;[0..8]);
    }
    read_codes_in_byte!(bytes[bytes.len() - 1];[0..8 - padding_bits]);

    decompressed_bytes
}

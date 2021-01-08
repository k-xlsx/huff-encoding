use super::{
    prelude::HuffTree,
    bitvec::prelude::BitVec,
};

use std::{
    fmt,
    convert::TryInto,
};




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

use std::collections::HashMap;

use bit_vec::BitVec;
use bitbit::{BitReader, BitWriter};


const EXTENSION: &str = "hfc";



pub fn get_header(tree_bin: &mut BitVec) -> BitVec{
    let tree_len: u64 = tree_bin.len() as u64;

    let mut bin_len = BitVec::new();
    for i in (0..64).rev(){
        let a = tree_len & (1 << i);
        match a > 0{
            true =>
                bin_len.push(true),
            false =>
                bin_len.push(false)
        }
    }
    bin_len.append(tree_bin);
    return bin_len;
}

pub fn get_encoded_string(s: &str, char_codes: &HashMap<char, BitVec>) -> BitVec{
    let mut encoded_str = BitVec::new();
    
    for c in s.chars(){
        let c_code = char_codes.get(&c).unwrap();
        for b in c_code{
            encoded_str.push(b);
        }
    }

    return encoded_str;
}

use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::fs;

use bit_vec::BitVec;

use crate::HuffTree;



const EXTENSION: &str = "hfe";



pub fn write_as_hfe<P: AsRef<Path>>(dir_path: P, file_name: &str, s: &str) -> io::Result<()>{
    //! Encode the string slice as Huffman code and write it to
    //! a .hfe file with the given name in the given dir_path
    //! 
    //! ## .hfe file structure
    //! ---
    //! * Header comprised of:
    //!   * 8 byte header length
    //!   * tree encoded in binary
    //! * Encoded text
    //! 
    //! # Examples
    //! ---
    //! ```
    //! use huff_encoding::file; 
    //! 
    //! fn main() -> std::io::Result<()> {
    //!     file::write_as_hfe("C:\\", "foo", "Lorem ipsum")?;
    //!     file::write_as_hfe("/home/user/", "bar", "dolor sit")?;
    //!     Ok(())
    //! }
    //! ```

    
    fn inner(path: &Path, s: &str) -> std::io::Result<()>{
        // construct huffman tree
        let tree = HuffTree::from(s);
        
        // encode string and get file header
        let mut h = get_header(&mut tree.to_bin());
        let mut es = get_encoded_string(s, tree.char_codes());

        // write header + encoded string as byte buffer
        let buffer = {h.append(&mut es); h.to_bytes()};
        return fs::write(path, buffer)
    }
    
    // ad name and extension to dir path
    let path = dir_path.as_ref().join(format!("{}.{}", file_name, EXTENSION));

    inner(&path, s.as_ref())
}


fn get_header(tree_bin: &mut BitVec) -> BitVec{
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

fn get_encoded_string(s: &str, char_codes: &HashMap<char, BitVec>) -> BitVec{
    let mut encoded_str = BitVec::new();
    
    for c in s.chars(){
        let c_code = char_codes.get(&c).unwrap();
        for b in c_code{
            encoded_str.push(b);
        }
    }

    return encoded_str;
}

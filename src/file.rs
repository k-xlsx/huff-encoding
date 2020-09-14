use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;
use std::{fs, io};
use io::Write;

use bit_vec::BitVec;

use crate::HuffTree;



const EXTENSION: &str = "hfe";



pub fn write_hfe<P: AsRef<Path>>(dir_path: P, file_name: &str, text: &str) -> io::Result<()>{
    //! Encode the string slice as Huffman code and write it to
    //! a .hfe file with the given name in the given dir_path
    //! 
    //! ## .hfe file structure
    //! ---
    //! * Byte containing the number of padding bits
    //! * Header comprised of:
    //!   * 8 byte header length
    //!   * tree encoded in binary
    //! * Encoded text
    //! 
    //! # Examples
    //! ---
    //! ```
    //! use huff_encoding::file::write_as_hfe; 
    //! 
    //! fn main() -> std::io::Result<()> {
    //!     write_as_hfe("C:\\", "foo", "Lorem ipsum")?;
    //!     write_as_hfe("/home/user/", "bar", "dolor sit")?;
    //!     Ok(())
    //! }
    //! ```

    
    fn inner(path: &Path, s: &str) -> io::Result<()>{
        // construct huffman tree
        let tree = HuffTree::from(s);
        
        // encode string and get file header
        let mut h = get_header(&mut tree.to_bin());
        let mut es = get_encoded_string(s, tree.char_codes());

        let padding_bits: u8 = {
            let n = (8 - (h.len() + es.len()) % 8) as u8; 
            match n{8 => 0, _ => n}
        };

        // header followed by encoded text, as bytes
        let contents_buffer = {h.append(&mut es); h.to_bytes()};

        // write padding bits, followed by the actual contents
        let mut file = fs::File::create(path)?;
        file.write_all(&[padding_bits])?;
        file.write_all(&contents_buffer)
    }
    
    // ad name and extension to dir path
    let path = dir_path.as_ref().join(format!("{}.{}", file_name, EXTENSION));

    inner(&path, text.as_ref())
}

pub fn read_hfe<P: AsRef<Path>>(path: P) -> io::Result<String>{
    //! Read text from a .hfe file
    //! 
    //! ## .hfe file structure
    //! ---
    //! * Byte containing the number of padding bits
    //! * Header comprised of:
    //!   * 8 byte header length
    //!   * tree encoded in binary
    //! * Encoded text
    //! 
    //! # Examples
    //! ---
    //! ```
    //! use huff_encoding::file::{write_as_hfe, read_from_hfe}; 
    //! 
    //! fn main() -> std::io::Result<()> {
    //!     write_as_hfe("/home/user/", "bar", "dolor sit")?;
    //!     let foo = read_from_hfe("/home/user/bar.hfe")?;
    //!     assert_eq!(&foo[..], "dolor sit");
    //!
    //!     Ok(())
    //! }
    //! ```

    fn inner(path: &Path) -> io::Result<String>{
        let bb = fs::read(path)?;

        let padding_bits = bb[0];

        let header_len = u64::from_be_bytes(bb[1..9].try_into().unwrap());
        let mut header = BitVec::from_bytes(&bb[9..]);
        let encoded_file = {
            let mut ef = header.split_off((header_len) as usize);
            for _ in 0..padding_bits{
                ef.pop();
            }
            ef
        };


        // check for errors in converting the header to coded_chars
        let coded_chars = HuffTree::coded_chars_from_bin(&header);
        if let Err(_) = coded_chars{
            return Err(io::Error::new(io::ErrorKind::Other, "Non UTF-8 char in header"));
        }
        let coded_chars = coded_chars.unwrap();
        
        let mut text = String::new();
        let mut current_code = BitVec::new();
        for b in encoded_file{
            current_code.push(b);
            
            // push to text if found char, else add to code 
            let c = coded_chars.get(&current_code);
            match c{
                None => (),
                Some(_) => {
                    text.push(*c.unwrap());
                    current_code = BitVec::new();
                }
            }
        }
        
        return Ok(text);
    }

    return inner(&path.as_ref())
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

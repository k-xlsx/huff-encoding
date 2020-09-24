use std::collections::HashMap;
use std::thread;
use std::convert::TryInto;
use std::path::Path;
use std::{fs, io};
use io::{BufWriter, Write};

use bit_vec::BitVec;

use crate::HuffTree;
use crate::utils::ration_vec;


const EXTENSION: &str = "hfe";



/// Encode the string slice as Huffman code and write it to
/// a .hfe file with the given name in the given dir_path
/// 
/// ## .hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> encoded contents padding bits
/// * Header comprised of:
///   * 8 byte header length
///   * HuffTree encoded in binary
/// * Encoded bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::file::write_as_hfe; 
/// 
/// fn main() -> std::io::Result<()> {
///     write_hfe("C:\\", "foo", "Lorem ipsum")?;
///     write_hfe("/home/user/", "bar", "dolor sit")?;
///     Ok(())
/// }
/// ```
pub fn write_hfe<P: AsRef<Path>>(dir_path: P, file_name: &str, bytes: &[u8]) -> io::Result<()>{
    fn inner(path: &Path, bytes: &[u8]) -> io::Result<()>{
        // construct huffman tree
        let tree = HuffTree::from(bytes);
        
        // encode string, get file header and calc their padding bits
        let h = get_header(&mut tree.to_bin());
        let es = get_encoded_bytes(bytes, tree.byte_codes().clone());
        let padding_bits = calc_padding_bits(es.len()) + (calc_padding_bits(h.len()) << 4);

    
        let file = fs::File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(&[padding_bits])?;
        buf_writer.write_all(&h.to_bytes())?;
        buf_writer.write_all(&es.to_bytes())
    }
    
    // add name and extension to dir path
    let path = dir_path.as_ref().join(format!("{}.{}", file_name, EXTENSION));

    inner(&path, bytes.as_ref())
}

/*
//TODO: Optimize this A LOT
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
    //!     write_hfe("/home/user/", "bar", "dolor sit")?;
    //!     let foo = read_hfe("/home/user/bar.hfe")?;
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
*/

/// Return a tree_bin preceded by its length
/// to be used as a .hfe file header.
fn get_header(tree_bin: &mut BitVec) -> BitVec{
    // get tree_bin.len() and add at the front of tree_bin
    let tree_len: u64 = tree_bin.len() as u64;
    let mut bin_len = BitVec::from_bytes(&tree_len.to_be_bytes());
    
    bin_len.append(tree_bin);
    return bin_len;
}

/// Return given bytes encoded with the given byte_codes HashMap
fn get_encoded_bytes(bytes: &[u8], byte_codes: HashMap<u8, BitVec>) -> BitVec{
    // allocate byte_codes onto the heap
    let byte_codes = Box::new(byte_codes);

    // divide the bytes into rations for threads to deal with 
    let byte_rations = ration_vec(bytes.to_vec(), num_cpus::get());

    // spawn threads encoding given bytes in ration
    let mut handles = vec![];
    for ration in byte_rations{
        let codes = byte_codes.clone();
        let handle = thread::spawn(move || {
            let mut encoded = BitVec::new();
            for byte in ration{
                let b_code = codes.get(&byte).unwrap();
                for bit in b_code{
                    encoded.push(bit);
                }
            }
            encoded

        });
        handles.push(handle);
    }

    // concatenate every encoded ration into encoded_bytes
    let mut encoded_bytes = BitVec::new();
    let mut encoded_to_concat: Vec<BitVec> = Vec::new();

    let mut i = 0;
    for handle in handles{
        if i == 0{
            encoded_bytes = handle.join().unwrap();
        }
        else{
            encoded_to_concat.push(handle.join().unwrap());
        }
        i += 1   
    }
    for encoded in encoded_to_concat.iter_mut(){
        encoded_bytes.append(encoded);
    }

    return encoded_bytes;
}

/// Return how many bits will be used as padding
/// with given the bit_count.
fn calc_padding_bits(bit_count: usize) -> u8{
    let n = (8 - bit_count % 8) as u8; 
    match n{8 => 0, _ => n}
}

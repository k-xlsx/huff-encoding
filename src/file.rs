use std::collections::HashMap;
use std::thread;
use std::convert::TryInto;
use std::path::Path;
use std::{fs, io};
use io::{BufWriter, Write};

use bitvec::prelude::{BitVec, LocalBits};

use crate::{HuffTree, HuffCode};
use crate::utils::{ration_vec, calc_padding_bits};


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
///   * 4 byte header length (in bytes)
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
        let tree = HuffTree::from_bytes(bytes);
        
        // encode string, get file header and calc their padding bits
        let h = get_header(&mut tree.to_bin());
        let es = get_encoded_bytes(bytes, tree.byte_codes().clone());
        let padding_bits = calc_padding_bits(es.len()) + (calc_padding_bits(h.len()) << 4);


        let file = fs::File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(&[padding_bits])?;
        buf_writer.write_all(&h.into_boxed_slice())?;
        buf_writer.write_all(&es.into_boxed_slice())?;

        Ok(())
    }
    
    // add name and extension to dir path
    let path = dir_path.as_ref().join(format!("{}.{}", file_name, EXTENSION));

    inner(&path, bytes.as_ref())
}

/// Read bytes encoded in a .hfe file
/// 
/// ## .hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> encoded contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree encoded in binary
/// * Encoded bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::file::{write_hfe, read_hfe}; 
/// 
/// fn main() -> std::io::Result<()> {
///     write_hfe("/home/user/", "bar", &"abbccc".as_bytes())?;
///     let foo = read_hfe("/home/user/bar.hfe")?;
///     assert_eq!(&foo[..], vec![97, 98, 98, 99, 99, 99]);
///
///     Ok(())
/// }
/// ```
pub fn read_hfe<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>>{
    fn inner(path: &Path) -> io::Result<Vec<u8>>{
        let raw_bytes = fs::read(path)?;

        let padding_bits = raw_bytes[0];
        let header_padding_bits =  padding_bits >> 4;
        let file_padding_bits = padding_bits & 0b0000_1111;

        let header_len = u32::from_be_bytes(raw_bytes[1..5].try_into().unwrap());
        let header = {
            let mut header_bits = BitVec::from_vec(raw_bytes[5..5 + header_len as usize].to_vec());
            header_bits.drain(header_bits.len() - (header_padding_bits as usize)..);
            header_bits
        };
        let coded_bytes = HuffTree::coded_chars_from_bin(&header);

        let encoded_file = {
            let file_bytes = &raw_bytes[5 + header_len as usize..];
            let mut file_bits: BitVec<bitvec::order::LocalBits, u8> = BitVec::from_vec(file_bytes.to_vec());
            file_bits.drain(file_bits.len() - (file_padding_bits as usize)..);
            file_bits
        };
        
        // TODO: Replace the hashmap here somehow
        let mut decoded_file: Vec<u8> = Vec::new();
        let mut current_code = BitVec::new();
        for bit in encoded_file{
            current_code.push(bit);
            let coded_byte = coded_bytes.get(&current_code);
            match coded_byte{
                Some(_) =>{
                    decoded_file.push(*coded_byte.unwrap());
                    current_code.clear();
                }
                None => (),
            }
        }

        Ok(decoded_file)
    }

    return inner(&path.as_ref())
}


/// Return a tree_bin preceded by its length
/// to be used as a .hfe file header.
fn get_header(tree_bin: &mut BitVec<LocalBits, u8>) -> BitVec<LocalBits, u8>{
    // get tree_bin.len() and add at the front of tree_bin
    let tree_len: u32 = ((tree_bin.len() + calc_padding_bits(tree_bin.len()) as usize) / 8) as u32;
    let mut bin_len = BitVec::from_vec(tree_len.to_be_bytes().to_vec());
    
    bin_len.extend_from_bitslice(&tree_bin[..]);
    return bin_len;
}

/// Return given bytes encoded with the given byte_codes HashMap
fn get_encoded_bytes(bytes: &[u8], byte_codes: HashMap<u8, HuffCode>) -> BitVec<LocalBits, u8>{
    // allocate byte_codes onto the heap
    let byte_codes = Box::new(byte_codes);

    // divide the bytes into rations for threads to deal with 
    let byte_rations = ration_vec(&bytes.to_vec(), num_cpus::get());

    // spawn threads encoding given bytes in ration
    let mut handles = Vec::with_capacity(num_cpus::get());
    for ration in byte_rations{
        let codes = byte_codes.clone();
        let handle = thread::spawn(move || {
            let mut encoded_chunk = BitVec::new();
            for byte in ration{
                let b_code = codes.get(&byte).unwrap();
                for bit in b_code{
                    encoded_chunk.push(bit);
                }
            }
            encoded_chunk
        });
        handles.push(handle);
    }

    // concatenate every encoded chunk into encoded_bytes
    // doing this is slow, but i've got no better idea
    // still faster than linear
    let mut encoded_bytes: BitVec<LocalBits, u8> = BitVec::with_capacity(3 * bytes.len() / 4);
    for handle in handles{	   
        encoded_bytes.extend_from_bitslice(&handle.join().unwrap()[..]);
    }

    return encoded_bytes;
}

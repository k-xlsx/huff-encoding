use std::collections::HashMap;
use std::thread;
use std::convert::TryInto;
use std::path::Path;
use std::{fs, io};
use io::{BufWriter, Write};

use bitvec::prelude::{BitVec, LocalBits};

use crate::{HuffTree, HuffCode};
use crate::utils::{ration_vec, calc_padding_bits};



/// Compress the string slice as Huffman code and write it to
/// a file with the given name (extension is arbitrary, but .hfe is recommended) 
/// in the given dir_path
/// 
/// Threaded version is faster for bigger files (huff_encoding::threaded_write_hfe).
/// 
/// ## hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> compressed contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree compressed in binary
/// * compressed bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::file::write_hfe; 
/// 
/// fn main() -> std::io::Result<()> {
///     write_hfe("C:\\", "foo", &"Lorem ipsum".as_bytes());
///     write_hfe("/home/user/", "bar", &"dolor sit".as_bytes());
///     Ok(())
/// }
/// ```
pub fn write_hfe<P: AsRef<Path>>(dir_path: P, file_name: &str, bytes: &[u8], ) -> io::Result<()>{
    return generic_write_hfe(dir_path, file_name, bytes, compress);
}

/// Compress the string slice as Huffman code and write it to
/// a file with the given name (extension is arbitrary, but .hfe is recommended) 
/// in the given dir_path, but using multiple threads (it's faster for bigger files).
/// 
/// Non-threaded version is faster for smaller files (huff_encoding::write_hfe).
/// 
/// ## hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> compressed contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree compressed in binary
/// * compressed bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::file::threaded_write_hfe; 
/// 
/// fn main() -> std::io::Result<()> {
///     threaded_write_hfe("C:\\", "foo", &"Lorem ipsum".as_bytes());
///     threaded_write_hfe("/home/user/", "bar", &"dolor sit".as_bytes());
///     Ok(())
/// }
/// ```
pub fn threaded_write_hfe<P: AsRef<Path>>(dir_path: P, file_name: &str, bytes: &[u8], ) -> io::Result<()>{
    return generic_write_hfe(dir_path, file_name, bytes, threaded_compress);
}

/// A generic version of write_hfe functions that accepts the used compress function as arg
fn generic_write_hfe<P: AsRef<Path>, F: FnOnce(&[u8]) -> Vec<u8>>(dir_path: P, file_name: &str, bytes: &[u8], compress_func: F) -> io::Result<()>{
    fn inner<F: FnOnce(&[u8]) -> Vec<u8>>(path: &Path, bytes: &[u8], compress_func: F) -> io::Result<()>{
        let compressed_bytes = compress_func(bytes);

        let file = fs::File::create(path)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write(&compressed_bytes)?;

        Ok(())
    }
    
    // add name and extension to dir path
    let path = dir_path.as_ref().join(file_name);

    inner(&path, bytes.as_ref(), compress_func)
}


/// Read bytes compressed in a huff compressed file
/// 
/// ## hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> compressed contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree compressed in binary
/// * compressed bytes
pub fn read_hfe<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>>{
    fn inner(path: &Path) -> io::Result<Vec<u8>>{
        let bytes = fs::read(path)?;
        Ok(decompress(&bytes))
    }

    return inner(&path.as_ref())
}

/// Returns given bytes compresses using 
/// huffman encoding.
/// 
/// Threaded version is faster for bigger files (huff_encoding::threaded_compress).
/// 
/// ## hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> compressed contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree compressed in binary
/// * compressed bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::compress; 
/// 
/// let foo = compress(&[97, 98, 98, 99, 99, 99]);
pub fn compress(bytes: &[u8]) -> Vec<u8>{
    return generic_compress(bytes, HuffTree::from_bytes(bytes), get_compressed_bytes);
}

/// Returns given bytes compresses using 
/// huffman encoding, but using multiple threads (it's faster for bigger files).
/// 
/// Non-threaded version is faster for smaller files (huff_encoding::compress).
/// 
/// ## hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> compressed contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree compressed in binary
/// * compressed bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::threaded_compress; 
/// 
/// let foo = threaded_compress(&[97, 98, 98, 99, 99, 99]);
/// ```
pub fn threaded_compress(bytes: &[u8]) -> Vec<u8>{
    return generic_compress(bytes, HuffTree::threaded_from_bytes(bytes), threaded_get_compressed_bytes);
}

// A generic version of the compress functions that accepts the tree and get_compressed_bytes func as arguments
fn generic_compress<F: FnOnce(&[u8], HashMap<u8, HuffCode>) -> BitVec<LocalBits, u8>>(bytes: &[u8], tree: HuffTree, get_compressed_bytes_func: F) -> Vec<u8>{
    // compress bytes, get file header and calc their padding bits
    let h = get_header(&mut tree.to_bin());
    let es = get_compressed_bytes_func(bytes, tree.byte_codes().clone());
    let padding_bits = calc_padding_bits(es.len()) + (calc_padding_bits(h.len()) << 4);


    let mut compressed_bytes: Vec<u8> = Vec::new();
    compressed_bytes.extend(&[padding_bits]);
    compressed_bytes.extend(h.into_boxed_slice().to_vec());
    compressed_bytes.extend(es.into_boxed_slice().to_vec());

    return compressed_bytes;
}

/// Return bytes decompressed from the given bytes
/// 
/// ## hfe file structure
/// ---
/// * Byte containing the number of padding bits
///   * first 4 digits -> header padding bits
///   * next 4 digits -> compressed contents padding bits
/// * Header comprised of:
///   * 4 byte header length (in bytes)
///   * HuffTree compressed in binary
/// * compressed bytes
/// 
/// # Examples
/// ---
/// ```
/// use huff_encoding::{compress, decompress}; 
/// 
/// let foo = compress(&[97, 98, 98, 99, 99, 99]);
/// let bar = decompress(&foo);
/// ```
pub fn decompress(bytes: &[u8]) -> Vec<u8>{
    return get_decoded_bytes(bytes);
}


/// Return a tree_bin preceded by its length
/// to be used as a hfe file header.
fn get_header(tree_bin: &mut BitVec<LocalBits, u8>) -> BitVec<LocalBits, u8>{
    // get tree_bin.len() and add at the front of tree_bin
    let tree_len: u32 = ((tree_bin.len() + calc_padding_bits(tree_bin.len()) as usize) / 8) as u32;
    let mut bin_len = BitVec::from_vec(tree_len.to_be_bytes().to_vec());
    
    bin_len.extend_from_bitslice(&tree_bin[..]);
    return bin_len;
}

/// Return given bytes compressed with the given byte_codes HashMap.
/// 
/// Threaded version is faster for bigger files.
fn get_compressed_bytes(bytes: &[u8], byte_codes: HashMap<u8, HuffCode>) -> BitVec<LocalBits, u8>{
    let mut compressed_bytes = BitVec::new();
    for byte in bytes{
        let b_code = byte_codes.get(&byte).unwrap();
        for bit in b_code{
            compressed_bytes.push(bit);
        }
    }

    return compressed_bytes;
}

/// Return given bytes compressed with the given byte_codes HashMap, but using
/// multiple threads (it's faster for bigger files).
/// 
/// Non-threaded version is faster for smaller files.
fn threaded_get_compressed_bytes(bytes: &[u8], byte_codes: HashMap<u8, HuffCode>) -> BitVec<LocalBits, u8>{
    // allocate byte_codes onto the heap
    let byte_codes = Box::new(byte_codes);

    // divide the bytes into rations for threads to deal with 
    let byte_rations = ration_vec(&bytes.to_vec(), num_cpus::get());

    // spawn threads encoding given bytes in ration
    let mut handles = Vec::with_capacity(num_cpus::get());
    for ration in byte_rations{
        let codes = byte_codes.clone();
        let handle = thread::spawn(move || {
            let mut compressed_chunk = BitVec::new();
            for byte in ration{
                let b_code = codes.get(&byte).unwrap();
                for bit in b_code{
                    compressed_chunk.push(bit);
                }
            }
            compressed_chunk
        });
        handles.push(handle);
    }

    // concatenate every compressed chunk into compressed_bytes
    // doing this is slow, but i've got no better idea
    // still faster than linear
    let mut compressed_bytes: BitVec<LocalBits, u8> = BitVec::with_capacity(3 * bytes.len() / 4);
    for handle in handles{	   
        compressed_bytes.extend_from_bitslice(&handle.join().unwrap()[..]);
    }

    return compressed_bytes;
}

// Return bytes decoded from given bytes.
fn get_decoded_bytes(bytes: &[u8]) -> Vec<u8>{
    // read how many bits were used for padding
    let padding_bits = bytes[0];
    let header_padding_bits =  padding_bits >> 4;
    let file_padding_bits = padding_bits & 0b0000_1111;

    // read coded bytes from header
    let header_len = u32::from_be_bytes(bytes[1..5].try_into().unwrap());
    let header = {
        let mut header_bits = BitVec::from_vec(bytes[5..5 + header_len as usize].to_vec());
        header_bits.drain(header_bits.len() - (header_padding_bits as usize)..);
        header_bits
    };
    let coded_bytes = HuffTree::coded_bytes_from_bin(&header);

    let compressed_file = {
        let file_bytes = &bytes[5 + header_len as usize..];
        let mut file_bits: BitVec<bitvec::order::LocalBits, u8> = BitVec::from_vec(file_bytes.to_vec());
        file_bits.drain(file_bits.len() - (file_padding_bits as usize)..);
        file_bits
    };
    
    // decode every byte
    // TODO: Replace the hashmap here somehow
    let mut decoded_bytes: Vec<u8> = Vec::new();
    let mut current_code = BitVec::<LocalBits, u8>::new();
    for bit in compressed_file{
        current_code.push(bit);
        let coded_byte = coded_bytes.get(&current_code);
        match coded_byte{
            Some(_) =>{
                decoded_bytes.push(*coded_byte.unwrap());
                current_code.clear();
            }
            None => (),
        }
    }

    return decoded_bytes;
}

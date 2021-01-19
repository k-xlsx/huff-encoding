use huff_coding::prelude::{
    compress_with_tree, 
    ByteWeights, 
    HuffTree,
};

use super::{
    utils,
    error::{
        Error,
        ErrorKind
    }
};

use std::{
    fs::File,
    convert::TryInto,
    path::PathBuf,
    io::{
        BufReader,
        BufWriter,
        Read,
        Write,
        Seek,
        SeekFrom,
    },
};



/// Read the the src file, compress it, and write the compressed data into dst file.
/// 
/// Chunk size means how many bytes will be read from src file at one time
pub fn read_compress_write(src_path: &PathBuf, dst_path: &PathBuf, chunk_size: usize) -> Result<(), Error>{
    // read from src file
    let src = File::open(src_path)?;
    let mut src_bytes_left = src.metadata().unwrap().len() as usize;
    let mut reader = BufReader::new(src);

    // write to dst file
    let dst = File::create(dst_path)?;
    let mut writer = BufWriter::new(dst);

    // allocate a u8 buffer of size == chunk_size
    let mut buf = vec![0; chunk_size];

    // create a HuffTree from the src file bytes
    let tree = huff_tree_from_reader(&mut reader, &mut src_bytes_left.clone(), &mut buf);
    let tree_bin = tree.as_bin();
    let tree_bin_padding = utils::calc_padding_bits(tree_bin.len());
    let tree_bin_bytes = tree_bin.into_vec();

    // return reader to start
    reader.seek(SeekFrom::Start(0))?;

    // write an empty byte, later to be filled by padding data
    writer.write_all(&[0])?;
    // write the tree_bin_bytes lenght as a 4 byte num
    writer.write_all(&(tree_bin_bytes.len() as u32).to_be_bytes())?;
    // write the HuffTree represented as bytes
    writer.write_all(&tree_bin_bytes)?;
    // compress and write compressed bytes, returning the number of bits used as padding
    let comp_padding = 
        compress_to_writer(
            &mut reader, &mut writer, 
            &mut src_bytes_left, &mut buf, 
            tree
        )?;

    // return to the start of the file and set the padding bits
    writer.seek(SeekFrom::Start(0))?;
    writer.write_all(&[(tree_bin_padding << 4) + comp_padding])?;

    writer.flush()?;
    Ok(())
}

/// Read the src file, decompress it, and write the decompressed data into dst file.
/// 
/// Chunk size means how many bytes will be read from src file at one time
pub fn read_decompress_write(src_path: &PathBuf, dst_path: &PathBuf, chunk_size: usize) -> Result<(), Error>{
    // read from src file
    let src = File::open(src_path)?;
    let mut src_bytes_left = src.metadata().unwrap().len() as usize;
    let reader = BufReader::new(src);

    // write to dst file
    let dst = File::create(dst_path)?;
    let mut writer = BufWriter::new(dst);

    // allocate a u8 buffer of size == chunk_size
    let mut buf = vec![0; chunk_size];

    // read only first 5 bytes
    let mut reader = reader.take(5);
    let bytes_read = reader.read(&mut buf)?;
    if bytes_read < 5{
        return Err(Error::new(
            format!("{:?} too short to decompress, missing header information", src_path),
            ErrorKind::MissingHeaderInfo
        ))
    }
    src_bytes_left -= 5;

    // read padding info from the first byte
    let padding = buf[0];
    let tree_padding_bits =  padding >> 4;
    let data_padding_bits = padding & 0b0000_1111;
    if tree_padding_bits > 7 || data_padding_bits > 7{
        return Err(Error::new(
            format!("{:?} stores invalid header information", src_path),
            ErrorKind::InvalidHeaderInfo
        ))
    }
    // read tree_bin's length
    let tree_len = u32::from_be_bytes(
        buf[1..5]
        .try_into()
        .unwrap()
    ) as usize;
    
    // read only next tree_len bytes
    reader.set_limit(tree_len as u64);
    let bytes_read = reader.read(&mut buf)?;
    if bytes_read < tree_len{
        return Err(Error::new(
            format!("{:?} too short to decompress, missing header information", src_path),
            ErrorKind::MissingHeaderInfo
        ))
    }
    src_bytes_left -= tree_len;

    // read the HuffTree
    let tree = match huff_coding::prelude::HuffTree::<u8>::try_from_bin({
        let mut b = huff_coding::bitvec::prelude::BitVec::from_vec(
            buf[..tree_len]
            .to_vec()
        );
        for _ in 0..tree_padding_bits{b.pop();}
        b
    }){
        Ok(tree) => tree,
        Err(_) => return Err(Error::new(
            format!("{:?} stores invalid header information", src_path), 
            ErrorKind::InvalidHeaderInfo
        ))
    };

    // decompress the remaining bytes
    let mut reader = reader.into_inner();
    decompress_to_writer(
        &mut reader, &mut writer, 
        &mut src_bytes_left, &mut buf,
        tree, data_padding_bits
    )?;

    writer.flush()?;
    Ok(())
}

/// Read bytes from reader, loading at most buf.len() bytes
/// from it at one time, building a HuffTree from them
pub fn huff_tree_from_reader<R: Read>(reader: &mut R, reader_bytes_left: &mut usize, buf: &mut [u8]) -> HuffTree<u8>{
    let mut bw = ByteWeights::new();
    while let Ok(_) = reader.read_exact(buf){
        bw += ByteWeights::threaded_from_bytes(&buf, 12);
        *reader_bytes_left -= buf.len();
    }
    if *reader_bytes_left > 0{
        bw += ByteWeights::threaded_from_bytes(&buf[..*reader_bytes_left], 12);
    }

    HuffTree::from_weights(bw)
}

/// Read bytes from reader, loading at most buf.len() bytes
/// from it at one time, compress them with the provided tree, 
/// and write them to writer
fn compress_to_writer<R: Read, W: Write + Seek>(
    reader: &mut R, writer: &mut W, 
    reader_bytes_left: &mut usize, buf: &mut [u8], 
    tree: HuffTree<u8>) -> Result<u8, Error>{
    let mut tree = tree;

    let mut prev_byte = 0;
    let mut prev_padding = 0;
    /// compress the buffer into CompressData, combining it with
    /// the prev_byte if the prev_padding != 0
    macro_rules! comp_data_from {
        ($buf:expr) => {{
            // get and own the compress data
            let (mut comp_bytes, padding_bits, huff_tree) = 
                compress_with_tree($buf, tree.clone())
                .unwrap()
                .into_inner();
            // if the previous compress data's padding isn't 0
            // write the comp_bytes minding the padding
            if prev_padding != 0{
                writer.seek(SeekFrom::Current(-1)).unwrap();

                comp_bytes = utils::offset_bytes(&comp_bytes, prev_padding as usize);
                comp_bytes[0] |= prev_byte
            }

            (comp_bytes, padding_bits, huff_tree)
        }};
    }
    // try to read exactly buf.len() bytes, compressing them and repeating
    while let Ok(_) = reader.read_exact(buf){
        let (comp_bytes, padding_bits, huff_tree) =  comp_data_from!(&buf);
        writer.write_all(&comp_bytes)?;
        
        prev_padding = padding_bits;
        prev_byte = comp_bytes[comp_bytes.len() - 1];
        tree = huff_tree;

        *reader_bytes_left -= buf.len();
    }
    // if couldn't read exactly buf.len() bytes and there are some bytes left, compress them
    if *reader_bytes_left > 0{
        let (comp_bytes, padding_bits, _) =  comp_data_from!(&buf[..*reader_bytes_left]);
        writer.write_all(&comp_bytes)?;

        prev_padding = padding_bits;
    }

    // return the written compressed data's padding bits
    Ok(prev_padding)
}

/// Read bytes from reader, loading at most buf.len() bytes
/// from it at one time, decompress them with the provided tree, 
/// and write them to writer
fn decompress_to_writer<R: Read, W: Write>(
    reader: &mut R, writer: &mut W, 
    reader_bytes_left: &mut usize, buf: &mut [u8],
    tree: HuffTree<u8>, padding_bits: u8) -> Result<(), Error>{

    // do pretty much the same thing as in huff_coding::comp::decompress
    // see it's docs for an explanation
    let mut decomp_buf = Vec::new();
    let mut current_branch = tree.root();
    macro_rules! read_codes_in_byte {
        ($byte: expr;[$bitrange:expr]) => {
            for bit_ptr in $bitrange{
                if current_branch.has_children(){
                    match ($byte >> (7 - bit_ptr)) & 1 == 1{
                        true =>{
                            current_branch = current_branch.right_child().unwrap();
                        }
                        false =>{
                            current_branch = current_branch.left_child().unwrap();
                        }
                    }
                }
                if !current_branch.has_children(){
                    decomp_buf.push(current_branch.leaf().letter().unwrap().clone());
                    current_branch = tree.root();
                }
            }
        };
    }
    // try to read exactly buf.len() bytes, decompressing them and writing
    while let Ok(_) = reader.read_exact(buf){
        for byte in &buf[..]{
            read_codes_in_byte!(byte;[0..8]);
        }
        writer.write_all(&decomp_buf)?;
        decomp_buf.clear();
        *reader_bytes_left -= buf.len();
    }
    // if couldn't read exactly buf.len() bytes and there are some bytes left, 
    // decompress them minding the padding bits
    if *reader_bytes_left > 0{
        for byte in &buf[..*reader_bytes_left - 1]{
            read_codes_in_byte!(byte;[0..8]);
        }
        read_codes_in_byte!(buf[*reader_bytes_left - 1];[0..8 - padding_bits]);
        writer.write_all(&decomp_buf)?;
    }
    Ok(())
}

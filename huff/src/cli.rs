use super::comp;

use std::{
    io,
    ffi::OsStr,
    path::{
        Path,
        PathBuf
    }
};



// TODO: move parsing paths into one or two funcs
// TODO: verbose errors

pub fn process_args(matches: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>>{
    let start = std::time::Instant::now();

    if matches.is_present("decompress"){
        on_decompress(
            std::path::PathBuf::from(matches.value_of("SRC_FILE").unwrap()),
            std::path::PathBuf::from(matches.value_of("DST_FILE").unwrap())
        )?;
    }
    else{
        on_compress(
            std::path::PathBuf::from(matches.value_of("SRC_FILE").unwrap()), 
            std::path::PathBuf::from(matches.value_of("DST_FILE").unwrap())
        )?;
    }

    if matches.is_present("time"){println!("{:?}", start.elapsed());}
    Ok(())
}

/// Things to do upon calling the compress command
fn on_compress(src_path: PathBuf, dst_path: PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    let (src_path, mut dst_path) = parse_paths(src_path, dst_path)?;
    dst_path.set_extension({
        let mut ex = dst_path
            .extension()
            .unwrap_or(OsStr::new("hff"))
            .to_os_string();
        if ex != "hff"{ex.push(".hff");}
        ex
    });

    let chunk_size = 2_000_000_000;

    comp::read_compress_write(src_path, dst_path, chunk_size)
}

/// Things to do upon calling the decompress command
fn on_decompress(src_path: PathBuf, dst_path: PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    let (src_path, mut dst_path) = parse_paths(src_path, dst_path)?;
    if src_path.extension() != Some(OsStr::new("hff")){
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "unrecognized file format")))
    }
    if dst_path.extension() == Some(OsStr::new("hff")){
        dst_path.set_extension("");
    }

    let chunk_size = 2_000_000_000;

    comp::read_decompress_write(src_path, dst_path, chunk_size)
}

fn parse_paths(src_path: PathBuf, dst_path: PathBuf) -> Result<(PathBuf, PathBuf), io::Error>{
    let mut dst_path = dst_path;

    // copy file name from src if none is provided
    if dst_path == Path::new("./"){
        dst_path.push(Path::new(src_path.file_name().unwrap()));
    }

    // check if dst is a file 
    if dst_path.is_dir(){
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "dst path is a directory"))
    }

    // check if src is a file
    if src_path.is_dir(){
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "src path is a directory"))
    }

    Ok((src_path, dst_path))
} 

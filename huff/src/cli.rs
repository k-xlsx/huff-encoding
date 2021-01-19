use super::{
    comp,
    error::{
        Error,
        ErrorKind,
    }
};

use std::{
    ffi::OsStr,
    path::{
        Path,
        PathBuf
    }
};



const EXTENSTION: &str = "hff";

// TODO: move parsing paths into one or two funcs

pub fn process_args(matches: clap::ArgMatches) -> Result<(), Error>{
    let start = std::time::Instant::now();

    // the decompress flag is present
    if matches.is_present("decompress"){
        on_decompress(
            std::path::PathBuf::from(matches.value_of("SRC_FILE").unwrap()),
            std::path::PathBuf::from(matches.value_of("DST_FILE").unwrap())
        )?;
    }
    // if no major flags are present, just compress
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
fn on_compress(src_path: PathBuf, dst_path: PathBuf) -> Result<(), Error>{
    // initial parsing
    let (src_path, mut dst_path) = parse_paths(src_path, dst_path)?;
    // add cli::EXTENSTION to the dst_path
    dst_path.set_extension({
        let mut ex = dst_path
            .extension()
            .unwrap_or(OsStr::new(EXTENSTION))
            .to_os_string();
        if ex != EXTENSTION{ex.push("."); ex.push(EXTENSTION);}
        ex
    });

    let chunk_size = 2_000_000_000;

    match comp::read_compress_write(&src_path, &dst_path, chunk_size){
        Ok(_) => Ok(()),
        Err(err) =>{
            // remove the dst file if it exists
            match std::fs::remove_file(dst_path){_ => ()};
            Err(err)
        }
    }
}

/// Things to do upon calling the decompress command
fn on_decompress(src_path: PathBuf, dst_path: PathBuf) -> Result<(), Error>{
    // initial parsing
    let (src_path, mut dst_path) = parse_paths(src_path, dst_path)?;
    // check if the src_path file has a cli::EXTENSTION extension
    if src_path.extension() != Some(OsStr::new(EXTENSTION)){
        return Err(Error::new(
            format!("Unrecognized file format, expected {}", EXTENSTION), 
            ErrorKind::UnrecognizedFormat
        ))
    }
    // remove the cli::EXTENSTION extension if the dst_path is the same as src_path
    if dst_path == {let mut p = PathBuf::from("./"); p.push(src_path.clone()); p}{
        dst_path.set_extension("");
    }

    let chunk_size = 2_000_000_000;

    match comp::read_decompress_write(&src_path, &dst_path, chunk_size){
        Ok(_) => Ok(()),
        Err(err) =>{
            // remove the dst file if it exists
            match std::fs::remove_file(dst_path){_ => ()};
            Err(err)
        }
    }
}


/// Check if the paths are files, and set dst_path to "./$src_path" if its equal to "./" 
fn parse_paths(src_path: PathBuf, dst_path: PathBuf) -> Result<(PathBuf, PathBuf), Error>{
    let mut dst_path = dst_path;

    // copy file name from src if none is provided
    if dst_path == Path::new("./"){
        dst_path.push(Path::new(src_path.file_name().unwrap()));
    }

    // check if dst is a file 
    if dst_path.is_dir(){
        return Err(Error::new(
            format!("{:?} is a directory", dst_path), 
            ErrorKind::NotFile
        ))
    }

    // check if src is a file
    if src_path.is_dir(){
        return Err(Error::new(
            format!("{:?} is a directory", src_path), 
            ErrorKind::NotFile
        ))
    }

    Ok((src_path, dst_path))
}

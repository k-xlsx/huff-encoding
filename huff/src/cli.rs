use super::{
    comp,
    error::{
        Error,
        ErrorKind,
    }
};

use std::{
    fs,
    process,
    ffi::OsStr,
    io::{
        self,
        Write,
    },
    path::{
        Path,
        PathBuf
    }
};



const EXTENSTION: &str = "hff";


macro_rules! parse_paths {
    ($src_path: expr, $dst_path:expr) =>{
        // copy file name from src if none is provided
        if $dst_path == Path::new("./SRC_FILE.hff"){
            $dst_path.set_file_name("");
            $dst_path.push(Path::new($src_path.file_name().unwrap()));
        }

        // check if dst is a file 
        if $dst_path.is_dir(){
            return Err(Error::new(
                format!("{:?} is a directory", $dst_path), 
                ErrorKind::NotFile
            ))
        }

        // check if src is a file
        if $src_path.is_dir(){
            return Err(Error::new(
                format!("{:?} is a directory", $src_path), 
                ErrorKind::NotFile
            ))
        }
    };
    (comp; $src_path: expr, $dst_path:expr) =>{
        parse_paths!($src_path, $dst_path);
        
        // add cli::EXTENSTION to the dst_path
        $dst_path = $dst_path.with_extension({
            let mut ex = $dst_path
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_os_string();
            if !ex.is_empty(){ex.push(".");}
            ex.push(EXTENSTION);
            ex
        });

    };
    (decomp; $src_path: expr, $dst_path:expr) =>{
        parse_paths!($src_path, $dst_path);
        // check if the src_path file has a cli::EXTENSTION extension
        if $src_path.extension() != Some(OsStr::new(EXTENSTION)){
            return Err(Error::new(
                format!("Unrecognized file format, expected {}", EXTENSTION), 
                ErrorKind::UnrecognizedFormat
            ))
        }
        // remove the cli::EXTENSTION extension if the dst_path is the same as src_path
        if $dst_path == {let mut p = PathBuf::from("./"); p.push($src_path.clone()); p}{
            $dst_path.set_extension("");
        }
    };
}

macro_rules! parse_block_size {
    ($block_size_str:expr) => {{
        let lowercase = $block_size_str.to_lowercase();
        let mut chars = lowercase.chars();
        let mut num = String::new();
        let mut mult = String::new();
        while let Some(c) = chars.next(){
            if c.is_ascii_digit(){num.push(c)}
            else{mult.push(c); break}
        }
        let num = match num.parse::<usize>(){
            Err(_) | Ok(0) =>
                return Err(Error::new(
                    String::from("Invalid block size"), 
                    ErrorKind::InvalidInput
                )),
            Ok(num) => num,
        };
        
        mult.push_str(&chars.collect::<String>());
        num * match &mult[..]{
            "" => 1,
            "k" => 1_000,
            "m" => 1_000_000,
            "g" => 1_000_000_000,
            "ki" => 1024,
            "mi" => 1_048_576,
            "gi" => 1_073_741_824,
            _ => 
                return Err(Error::new(
                    String::from("Invalid block size"), 
                    ErrorKind::InvalidInput
                )),
        }
    }};
}

macro_rules! ask_replace {
    ($path: expr, $noask:expr) => {
        if $path.exists() && !$noask{
            print!("{:?} already exists, do you want to replace it? [Y/N]: ", $path);
            io::stdout().flush()?;

            let mut yes_no = String::new();
            io::stdin().read_line(&mut yes_no)?;
            match &yes_no.to_lowercase()[..yes_no.len() - 1]{
                "yes" | "y" => {println!(); ()},
                "no" | "n" => process::exit(0),
                _ => 
                    return Err(Error::new(
                        String::from("Invalid input [Y or N]"), 
                        ErrorKind::InvalidInput)
                    ),
            }
        }
    };
}


pub fn process_args(matches: clap::ArgMatches) -> Result<(), Error>{
    let start = std::time::Instant::now();

    let src_path = std::path::PathBuf::from(matches.value_of("SRC_FILE").unwrap());
    let mut dst_path = std::path::PathBuf::from(matches.value_of("DST_FILE").unwrap());
    
    let block_size = parse_block_size!(matches.value_of("block-size").unwrap());

    // the decompress flag is present
    if matches.is_present("decompress"){
        parse_paths!(decomp; src_path, dst_path);
        // ask if should replace dst_file
        ask_replace!(dst_path, matches.is_present("noask"));
        // read src, decompress it, write the results to dst
        comp::read_decompress_write(&src_path, &dst_path, block_size)?;
    }
    // if no major flags are present, just compress
    else{
        parse_paths!(comp; src_path, dst_path);
        // ask if should replace dst_file
        ask_replace!(dst_path, matches.is_present("noask"));
        // read src, compress it, write the results to dst
        comp::read_compress_write(&src_path, &dst_path, block_size)?;
    }
    if matches.is_present("replace"){
        fs::remove_file(src_path).unwrap();
    }

    if matches.is_present("time"){println!("{:?}", start.elapsed());}
    Ok(())
}

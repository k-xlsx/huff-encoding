use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use huff_encoding::file::{write_hfe, threaded_write_hfe, read_hfe};


#[derive(StructOpt)]
struct Cli{
    /// Show the time it took for a command to finish
    #[structopt(short = "t", long = "time")]
    time: bool,

    #[structopt(subcommand)]
    cmd: Commands,
}

#[derive(StructOpt)]
#[structopt(about = "kinda wonky compressing software")]
pub enum Commands {
    /// Compress the file from src_path into dst_path.
    /// 
    /// If no dst_path is provided, the compressed file
    /// is saved with the src_path's file name and a 'hfe' extension.
    Compress{
        /// Path to the file you want to compress.
        #[structopt(parse(from_os_str))]
        src_path: PathBuf,

        /// Path where to put the compressed file 
        /// (default file name is the same as src + a 'hfe' extension).
        #[structopt(parse(from_os_str), default_value = "./")]
        dst_path: PathBuf,
        
        /// Use only one thread to compress (Can be faster for smaller files).
        #[structopt(short = "s", long = "single_thread")]
        single_thread_flag: bool,
    },
    /// Decompress the file from src_path into dst_path.
    /// (with the extension stored in the compressed file).
    /// 
    /// If no dst_path is provided, the decompressed file
    /// is saved with the src_path's file name.
    Decompress {
        /// Path to the file you want to decompress.
        #[structopt(parse(from_os_str))]
        src_path: PathBuf,

        /// Path where to put the decompressed file.
        /// (default file name is the same as src + extension stored in compressed file).
        #[structopt(parse(from_os_str), default_value = "./")]
        dst_path: PathBuf,
    },
}


pub fn process_args() -> Result<(), &'static str>{
    let cli = Cli::from_args();

    match cli.cmd{
        Commands::Compress{src_path, dst_path, single_thread_flag} =>{
            let start = std::time::Instant::now();

            let mut dst_path = dst_path;
            
            // check if src exists and is a file
            if !src_path.exists() || !src_path.is_file(){
                return Err("src path not found")
            }

            // copy file name from src if none is provided
            if dst_path == Path::new("./"){
                dst_path.push(Path::new(src_path.file_name().unwrap()).with_extension("hfe"));
            }

            // check if path to dst exists
            if !dst_path.parent().unwrap().exists() && 
                dst_path.parent().unwrap() != Path::new(""){
                return Err("dst path not found")
            }
            // check if dst is a file 
            if dst_path.is_dir(){
                return Err("dst is a directory")
            }
    

            // read src
            let tx = spawn_wait_indicator("reading src file", Duration::from_millis(800));
            let src_bytes = fs::read(&src_path).unwrap();
            let _ = tx.send(());
            println!();
                
            // extract dst_name & dst_path
            let dst_name = dst_path.file_name().unwrap();
            let dst_path = match dst_path.parent(){Some(_) => dst_path.parent().unwrap(), None => Path::new("")};
    
            // compress
            let tx = spawn_wait_indicator("compressing", Duration::from_millis(800));
            // single-threaded
            if single_thread_flag{
                write_hfe(&dst_path, &Path::new(dst_name), src_path.extension(), &src_bytes).unwrap();
            }
            // multi-threaded
            else{
                threaded_write_hfe(&dst_path, &Path::new(dst_name), src_path.extension(), &src_bytes).unwrap();
            }
            let _ = tx.send(());
            
            // print time if t flag
            print!("\ndone.");
            if cli.time{
                let elapsed = start.elapsed();
                print!(" {:?}", elapsed);
            }
            println!();
        },
        Commands::Decompress{src_path, dst_path} =>{
            let start = std::time::Instant::now();

            let mut dst_path = dst_path;

            // check if src exists and is a file
            if !src_path.exists() || !src_path.is_file(){
                return Err("src path not found")
            }

            // copy file name from src if none is provided
            if dst_path == Path::new("./"){
                dst_path.push(Path::new(src_path.file_name().unwrap()));
            }

            // check if path to dst exists
            if !dst_path.parent().unwrap().exists() && 
                dst_path.parent().unwrap() != Path::new(""){
                return Err("dst path not found")
            }
            // check if dst is a file 
            if dst_path.is_dir(){
                return Err("dst is a directory")
            }
    

            // decompress src
            let tx = spawn_wait_indicator("decompressing", Duration::from_millis(1000));
            let decompress_result = read_hfe(src_path).unwrap();
            let _ = tx.send(());
            println!(".");
    
            // write decompressed to dst with the read extension
            let tx = spawn_wait_indicator("writing to destination", Duration::from_millis(1000));
            fs::write(dst_path.with_extension(decompress_result.extension()), decompress_result.bytes()).unwrap();
            let _ = tx.send(());
            
            // print time if t flag
            print!("\ndone");
            if cli.time{
                let elapsed = start.elapsed();
                print!(" {:?}", elapsed);
            }
            println!(".");
        },
    }

    return Ok(());
}

fn spawn_wait_indicator(msg: &'static str, delay: Duration) -> mpsc::Sender<()> {
    let (tx, rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel();
    thread::spawn(move || {
        let mut dots: u8 = 3;
        loop{
            let _ = io::stdout().flush();
            print!("{}", msg);
            for _ in 0..dots{print!(".")}
            for _ in 0..3-dots{print!(" ")}
            thread::sleep(delay);
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            dots = if dots == 3{1}else{dots + 1};
            print!("\r");
        }
    });

    return tx;
}

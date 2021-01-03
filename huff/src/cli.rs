
use structopt::StructOpt;

use std::{
    thread,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::Duration,
    sync::mpsc::{self, TryRecvError},
};



#[derive(StructOpt)]
struct Cli{
    /// Show the time it took for a command to finish
    #[structopt(short = "t", long = "time")]
    time: bool,

    #[structopt(subcommand)]
    cmd: Commands,
}

#[derive(StructOpt)]
#[structopt(about = "kinda wonky compression software")]
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

    // TODO: cli for binary
    match cli.cmd{
        Commands::Compress{src_path, dst_path, single_thread_flag} =>{
            todo!();
        },
        Commands::Decompress{src_path, dst_path} =>{
            todo!();
        }
    }
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

    tx
}

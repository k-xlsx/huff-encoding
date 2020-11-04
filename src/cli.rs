use std::path::PathBuf;

use structopt::StructOpt;



#[derive(StructOpt)]
#[structopt(about = "kinda slow compressing software")]
pub enum Cli {
    Compress {
        #[structopt(parse(from_os_str))]
        src_path: PathBuf,

        #[structopt(parse(from_os_str), default_value = ".")]
        dst_path: PathBuf,
    },
    Decompress {
        #[structopt(parse(from_os_str))]
        src_path: PathBuf,

        #[structopt(parse(from_os_str), default_value = ".")]
        dst_path: PathBuf,
    },
}

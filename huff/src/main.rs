/// Functions parsing and processing args
mod cli;
/// error returned by the program
mod error;
/// Functions reading file, compressing/decompressing them, 
/// and writing the results to file
mod comp;
/// Various utility functions
mod utils;

// TODO: add multithreading

fn main() -> Result<(), error::Error>{
    let yaml = clap::load_yaml!("../res/cli.yml");
    let app = clap::App::from_yaml(
        yaml
    )
    .name(clap::crate_name!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!());

    match cli::process_args(app.get_matches()){
        Ok(_) => Ok(()),
        Err(err) => err.exit()
    }
}

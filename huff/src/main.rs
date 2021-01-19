//! kinda wonky compression software
// TODO: docs

mod cli;
/// Functions reading file, compressing/decompressing them, 
/// and writing the results to file
mod comp;
/// Various utility functions
mod utils;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let yaml = clap::load_yaml!("../res/cli.yml");
    let app = clap::App::from_yaml(
        yaml
    );

    cli::process_args(app.get_matches())
}

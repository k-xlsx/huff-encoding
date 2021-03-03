use huff::{
    error,
    cli
};

fn main() -> Result<(), error::Error>{
    let yaml = clap::load_yaml!("../res/cli.yml");
    let app = clap::App::from_yaml(
        yaml
    )
    .name(clap::crate_name!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!());

   cli::process_args(app.get_matches())
}

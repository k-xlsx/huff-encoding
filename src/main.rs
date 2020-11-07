mod cli;

fn main() -> Result<(), &'static str>{
    cli::process_args()
}

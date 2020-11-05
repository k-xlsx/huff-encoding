mod cli;

fn main() -> Result<(), &'static str>{
    return cli::process_args()
}

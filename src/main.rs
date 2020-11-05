use structopt::StructOpt;


mod cli;



fn main(){
    let start = std::time::Instant::now();
    println!("START");
    //---------------------------\\

    let cli = cli::Cli::from_args();

    //---------------------------\\
    let elapsed = start.elapsed();
    println!("{:?}\nEND", elapsed);
}

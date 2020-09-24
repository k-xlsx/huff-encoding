use huff_encoding;
use std::time;

 
//TODO: tests

fn main(){
    let start = time::Instant::now();
    println!("START");
    //---------------------------


    huff_encoding::file::write_hfe("", "test", &"Hello, World!".as_bytes()).expect("file write error");


    //---------------------------
    let elapsed = start.elapsed();
    println!("{:?}\nEND", elapsed);
}

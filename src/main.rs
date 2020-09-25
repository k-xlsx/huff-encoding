use huff_encoding;
use std::time;

 
// TODO: tests

fn main(){
    let start = time::Instant::now();
    println!("START");
    //---------------------------


    huff_encoding::file::write_hfe("", "test", &"Hello, World!".as_bytes()).expect("file write error");
    let decoded_bytes = huff_encoding::file::read_hfe("test.hfe").expect("file read error");
    println!("{:?}", decoded_bytes);

    //---------------------------
    let elapsed = start.elapsed();
    println!("{:?}\nEND", elapsed);
}

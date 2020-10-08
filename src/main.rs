use huff_encoding;
use std::time;

 
// TODO: tests

fn main(){
    let start = time::Instant::now();
    println!("START");
    //---------------------------\\


    let s = String::from("Hello, World!aaą😎");

    huff_encoding::file::write_hfe("", "test", &s.as_bytes()).expect("file write error");
    let decoded_bytes = huff_encoding::file::read_hfe("test.hfe").expect("file read error");

    println!("{:?}", s);
    println!("{:?}", s.as_bytes());
    println!("{:?}", std::str::from_utf8(&decoded_bytes));
    println!("{:?}", decoded_bytes);


    //---------------------------\\
    let elapsed = start.elapsed();
    println!("{:?}\nEND", elapsed);
}

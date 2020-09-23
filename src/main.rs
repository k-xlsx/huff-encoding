use huff_encoding;
use std::time;
use std::fs;

 
//TODO: tests

fn main(){
    let start = time::Instant::now();
    println!("START\n{:?}", start);
    //---------------------------


    let bytes = fs::read("test_enwik9").unwrap();

    // let tree = huff_encoding::HuffTree::from(&bytes);
    // println!("{:?}", tree);
    // huff_encoding::file::get_encoded_bytes(&bytes, tree.byte_codes().clone());

    huff_encoding::file::write_hfe("", "test", &bytes).expect("file write error");


    //---------------------------
    let elapsed = start.elapsed();
    println!("{:?}\nEND", elapsed);
}


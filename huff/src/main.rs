use huff_coding::prelude::*;



fn main() {
    let bytes = b"abbccc";

    let start = std::time::Instant::now();

    let c = compress(bytes);

    let d = decompress(&c);
        
    assert_eq!(&d.unwrap(), bytes);
   

    println!("\n{:?}", start.elapsed());
}

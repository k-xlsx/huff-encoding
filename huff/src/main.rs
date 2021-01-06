use huff_coding::prelude::*;



fn main() {
    let bytes = b"abbccc";
    let start = std::time::Instant::now();

    for b in compress(bytes){
        print!("{:08b}", b);
    }

    println!("\n{:?}", start.elapsed());
}

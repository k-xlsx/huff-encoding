use huff_coding::prelude::*;
use std::io::prelude::*;

fn main() {
    let bf = ByteFreqs::threaded_from_bytes(b"abbccc", 12);
    let start = std::time::Instant::now();

    let t = HuffTree::from_freq(bf);
    let tb = HuffTree::<u8>::try_from_bin(t.as_bin()).unwrap();

    println!("{:#?}\n{:?}", tb.read_codes(), start.elapsed());
}

use huff_coding::prelude::*;

fn main() {
    let start = std::time::Instant::now();
    let t = HuffTree::from_freq(ByteFreqs::threaded_from_bytes(&std::fs::read("benchmarks.tmp/enwik9").unwrap(), 12));

    let x = HuffTree::<u8>::try_from_bin(t.as_bin());

    let end = start.elapsed();
    println!("{:#?}\n{:?}", x, end);
}

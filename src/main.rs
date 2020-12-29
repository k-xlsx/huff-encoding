use huff_coding::prelude::*;

fn main() {
    let t = HuffTree::from_freq(ByteFreqs::threaded_from_bytes(b"abbccc", 12));
    println!("{:#?}", t.as_bin());
}

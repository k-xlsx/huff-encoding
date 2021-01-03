use huff_coding::prelude::*;



fn main() {
    let bw = ByteWeights::threaded_from_bytes(b"abbccc", 12);
    let start = std::time::Instant::now();

    let t = HuffTree::from_weights(bw);
    let tb = HuffTree::<u8>::try_from_bin(t.as_bin()).unwrap();

    println!("{:#?}\n{:?}", tb.read_codes(), start.elapsed());
}

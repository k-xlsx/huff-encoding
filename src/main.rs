use huff_coding::prelude::*;

fn main() {
    let t = HuffTree::from_freq(ByteFreqs::from_bytes(b"abbccc"));

    let x = HuffTree::<u8>::try_from_bin(t.as_bin());

    println!("{:#?}", x);
}

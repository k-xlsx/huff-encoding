

use huff_coding::prelude::*;

fn main() -> Result<(), &'static str>{
    let b = b"abbccc";
    let start = std::time::Instant::now();

    
        let c = compress_with_tree(b, HuffTree::from_weights(ByteWeights::from_bytes(b))).unwrap();
        let d = decompress(&c);
    
        assert_eq!(b.to_vec(), d);
        for by in c.to_bytes(){
            print!("{:#010b} ", by);
        }

    println!("{:?}", start.elapsed());
    Ok(())
}

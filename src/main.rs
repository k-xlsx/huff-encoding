use std::char;
use std::collections::HashMap;
use bit_vec::BitVec;
use huff_encoding::huff_structs;



fn main(){
    let s = "abbccc";

    let tree = huff_structs::HuffTree::from(s);

    println!("{:#?}\n", tree.as_bin());


    println!("{:#?}", tree.char_codes());
    println!("{:#?}", huff_structs::HuffTree::char_codes_from_bin(&tree.as_bin()));
}

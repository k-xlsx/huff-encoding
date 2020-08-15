use huff_encoding::huff_structs;
use huff_encoding::file_rw::hfc_prefix;



fn main(){
    let s = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff";

    let tree = huff_structs::HuffTree::from(s);
    println!("{:#?}", tree.char_codes());

    println!("{}", hfc_prefix(&tree));
}

mod huff_structs;

fn main(){
    let tree = huff_structs::HuffTree::from(&huff_structs::get_chars_to_freq(&String::from("aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff")));
    println!("{:?}", tree);
}

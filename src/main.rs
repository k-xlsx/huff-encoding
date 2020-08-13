mod huff_structs;

fn main(){
    let tree = huff_structs::HuffTree::from(&String::from("aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff"));
    println!("{:?}", tree.root());
}

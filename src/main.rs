use huff_encoding::huff_structs;

fn main(){
    let s = "Hello, World!";

    let tree = huff_structs::HuffTree::from(s);
    println!("{:#?}", tree);
}

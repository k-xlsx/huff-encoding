mod huff_structs;


// TODO: add tests

fn main() {
    let s: String = String::from("Hello, World!");
    let chars_to_freq = huff_structs::get_chars_to_freq(&s);

    let huff_tree = huff_structs::HuffTree::new(chars_to_freq);
    println!("{:?}", huff_tree)
}

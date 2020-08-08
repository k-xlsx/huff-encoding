mod huff_code;
mod huff_structs;


// TODO: add tests

fn main() {
    println!("\nSTART");

    let s: String = String::from("aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff");
    let (huf_tree, huff_root) = huff_code::build_huff_tree(&s);

    // print branches and root; TODO: make it look nicer

    println!("root: {}", &huff_root.get_frequency());
    for (parent, children) in &huf_tree {
        println!("{}: {}", match parent.get_character(){Some(_) => parent.get_character().unwrap(), None => '\0'}, parent.get_frequency());
        for leaf in children{
            println!("\t{}: {}", match leaf.get_character(){Some(_) => leaf.get_character().unwrap(), None => '\0'}, leaf.get_frequency());
        }
    }

    println!("\nEND");
}

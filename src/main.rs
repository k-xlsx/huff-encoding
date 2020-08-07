mod huf_code;



fn main() {
    println!("\nSTART");

    let s: String = String::from("Hello, World!");
    let huf_tree = huf_code::build_huff_tree(&s);

    // print branches and root; TODO: make it look nicer
    println!("root: {}", &huf_tree.1.frequency);
    for (key, value) in &huf_tree.0 {
        println!("{}: {}", match key.character{Some(_) => key.character.unwrap(), None => '\0'}, key.frequency);
        for leaf in value{
            println!("\t{}: {}", match leaf.character{Some(_) => leaf.character.unwrap(), None => '\0'}, leaf.frequency);
        }
    }

    println!("\nEND");
}

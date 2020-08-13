
mod leaf;
mod branch;
mod branch_heap;
mod tree;



pub use self::leaf::HuffLeaf;
pub use self::branch::HuffBranch;
pub use self::tree::HuffTree;


use std::collections::HashMap;



pub fn get_chars_to_freq(s: &str) -> HashMap<char, u32>{
    let mut ctf: HashMap<char, u32> = HashMap::new();

    for c in s.chars(){
        let cf_entry = ctf.entry(c).or_insert(0);
        *cf_entry += 1;
    }

    return ctf;
}

mod leaf;
mod branch;
mod branch_heap;
mod tree;


pub use self::leaf::HuffLeaf;
pub use self::branch::HuffBranch;
pub use self::tree::HuffTree;

use std::collections::HashMap;



pub fn chars_to_freq(s: &str) -> HashMap<char, usize>{
    let mut ctf: HashMap<char, usize> = HashMap::new();

    for c in s.chars(){
        let cf_entry = ctf.entry(c).or_insert(0);
        *cf_entry += 1;
    }

    return ctf;
}

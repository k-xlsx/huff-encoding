//

mod leaf;
mod tree;
mod leaf_vec;


pub use self::leaf::HuffLeaf;
pub use self::tree::HuffTree;

use std::collections::HashMap;



pub fn get_chars_to_freq(s: &String) -> HashMap<char, u32>{
    let mut chars_to_freq: HashMap<char, u32> = HashMap::new();

    for c in s.chars(){
        let cf_entry = chars_to_freq.entry(c).or_insert(0);
        *cf_entry += 1;
    }

    return chars_to_freq;
}

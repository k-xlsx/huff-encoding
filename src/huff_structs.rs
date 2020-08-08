mod leaf;
mod tree;
mod leaf_vec;


pub use self::leaf::HuffLeaf;
pub use self::leaf_vec::HuffLeafVec;
pub use self::tree::HuffTree;

use std::collections::HashMap;



/// Returns a HashMap with keys being chars found in the given String
/// and the values being the number of their appearances.
pub fn get_chars_to_freq(s: &String) -> HashMap<char, u32>{
    let mut chars_to_freq: HashMap<char, u32> = HashMap::new();

    for c in s.chars(){
        // insert current char if not in HashMap, then increment its freq
        let cf_entry = chars_to_freq.entry(c).or_insert(0);
        *cf_entry += 1;
    }

    return chars_to_freq;
}

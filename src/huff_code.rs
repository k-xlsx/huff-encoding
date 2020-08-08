// FIXME: just fix this whole file

use super::huff_structs::HuffLeaf;
use super::huff_structs::HuffLeafVec;
use super::huff_structs::HuffTree;

use std::collections::HashMap;


/// Builds a Huffman Encoding tree from the given String.  
/// 
/// Returns a HashMap representing the tree and the tree's root. 
pub fn build_huff_tree(s: &String) -> (HashMap<HuffLeaf, [HuffLeaf; 2]>, HuffLeaf){
    let chars_to_freq = build_char_to_freq(&s);

    let mut huff_tree = HuffTree::new();
    let mut leaf_vec = HuffLeafVec::new(&chars_to_freq);

    while leaf_vec.len() > 1{
        let min_pair = leaf_vec.get_min_pair().unwrap();
        leaf_vec.drain_min_pair();

        let joint_leaf = HuffLeaf::new(None, min_pair[0].get_frequency() + min_pair[1].get_frequency());
        leaf_vec.push(joint_leaf);
        
        huff_tree.add(joint_leaf, min_pair)
    }
    let root = huff_tree.get_root();

    assert!(root != None);
    return (huff_tree.get_tree(), root.unwrap());
}


/// Returns a HashMap with keys being chars found in the given String
/// and the values being the number of their appearances.
fn build_char_to_freq(s: &String) -> HashMap<char, u32>{
    let mut chars_to_freq: HashMap<char, u32> = HashMap::new();

    for c in s.chars(){
        // insert current char if not in HashMap, then increment its freq
        let cf_entry = chars_to_freq.entry(c).or_insert(0);
        *cf_entry += 1;
    }

    return chars_to_freq;
}

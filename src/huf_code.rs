use std::collections::HashMap;
use std::option::Option;



/// Struct representing a node in the Huffman 
/// Encoding tree.  
/// 
/// Contains the stored character (can be None to represent
/// joint nodes), the frequency and an id.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct HuffLeaf{
    pub character: Option<char>,
    pub frequency: u32,

    id: usize
}


/// Builds a Huffman Encoding tree from the given String.  
/// 
/// Returns a HashMap representing the tree and the tree's root. 
pub fn build_huff_tree(s: &String) -> (HashMap<HuffLeaf, [HuffLeaf; 2]>, HuffLeaf){
    let chars_to_freq = build_char_to_freq(&s);

    let mut huff_tree: HashMap<HuffLeaf, [HuffLeaf; 2]> = HashMap::new();
    let mut root: HuffLeaf = HuffLeaf{character: None, frequency: 0, id: 0};

    let mut leaf_heap: Vec<HuffLeaf> = build_leaf_heap(&chars_to_freq);


    while leaf_heap.len() > 1{
        // get lowest frequency pair and remove it from the heap
        let min_leaf_pair: [HuffLeaf; 2] = get_min_leaf_pair(&leaf_heap);
        for min_leaf in min_leaf_pair.iter(){
            leaf_heap.remove(find_leaf_index(&min_leaf, &leaf_heap).unwrap());
        }

        // make a joint leaf by summing frequencies of the min_leaf_pair
        // and push it onto the heap.
        let joint_leaf = HuffLeaf{
            character: None,
            frequency: min_leaf_pair[0].frequency + min_leaf_pair[1].frequency,
            id: std::usize::MAX - leaf_heap.len()
        };
        leaf_heap.push(joint_leaf);
        
        // insert the joint leaf with the pair as 'children' into the tree
        huff_tree.insert(joint_leaf, min_leaf_pair);
        root = joint_leaf
    }

    // check if root has been set just in case
    assert!(root.frequency != 0, "root has not been found");
    return (huff_tree, root)
}


/// Returns a Vector of HuffLeaves made from char_to_freq
fn build_leaf_heap(chars_to_freq: &HashMap<char, u32>) -> Vec<HuffLeaf>{
    let mut leaf_heap: Vec<HuffLeaf> = Vec::new();

    let mut i = 0;
    for (c, f) in chars_to_freq{
        leaf_heap.push(HuffLeaf{
            character: Some(*c), 
            frequency: *f,
            id: i
            }
        );
        i += 1
    }

    return leaf_heap
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


/// Returns a pair of leaves from the leaf_heap
/// with the lowest frequencies.
fn get_min_leaf_pair(leaf_heap: &Vec<HuffLeaf>) -> [HuffLeaf; 2]{
    // check if a pair is possible to avoid weird errors
    assert!(leaf_heap.len() > 1, "leaf_heap must contain at least 2 leaves");

    // set the pair to None at start
    let mut min_leaf_pair: [Option<HuffLeaf>; 2] = [None, None];

    for leaf in leaf_heap{

        let min_leaf = min_leaf_pair[0].get_or_insert(*leaf);

        // if the min_leaf has just been set
        if min_leaf.id == leaf.id{
            continue
        }
        else if leaf.frequency <= min_leaf.frequency{
            // set new min and set previous min on index 1
            min_leaf_pair = [Some(*leaf), Some(*min_leaf)]
        }
        else{
            let min_leaf = min_leaf_pair[1].get_or_insert(*leaf);

            if leaf.frequency <= min_leaf.frequency{
                min_leaf_pair[1] = Some(*min_leaf)
            }
        }
    }

    return [min_leaf_pair[0].unwrap(), min_leaf_pair[1].unwrap()]
}


/// Returns the index of the given leaf in the leaf_heap 
fn find_leaf_index(leaf: &HuffLeaf, leaf_heap: &Vec<HuffLeaf>) -> Option<usize>{
    for (i, l) in leaf_heap.iter().enumerate(){
        if l.id == leaf.id{
            return Some(i)
        }
    }

    return None
}

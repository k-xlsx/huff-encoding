#![allow(dead_code)]


use std::collections::HashMap;
use crate::huff_structs::{leaf::HuffLeaf, leaf_vec::HuffLeafVec};

//TODO: make HuffTree an actual tree

#[derive(Debug)]
pub struct HuffTree{
    tree: HashMap<HuffLeaf, [HuffLeaf; 2]>,
    root: Option<HuffLeaf>
}

impl HuffTree{
    pub fn new(chars_to_freq: HashMap<char, u32>) -> HuffTree{
        assert!(chars_to_freq.len() > 1, "no huffman tree for single char");

        let mut huff_tree = HuffTree{
            tree: HashMap::new(),
            root: None
        };
        huff_tree.build_tree(chars_to_freq);

        return huff_tree;
    }

    pub fn add(&mut self, root: HuffLeaf, branches: [HuffLeaf; 2]){
        self.tree.insert(root, branches);
        self.root = Some(root);
    }

    pub fn get_root(&self) -> Option<HuffLeaf>{
        return self.root;
    }

    pub fn get_tree(self) -> HashMap<HuffLeaf, [HuffLeaf; 2]>{
        return self.tree;
    }


    fn build_tree(&mut self, chars_to_freq: HashMap<char, u32>){   
        let mut leaf_vec = HuffLeafVec::new(&chars_to_freq);
        
        while leaf_vec.len() > 1{
            let min_pair = leaf_vec.get_min_pair().unwrap();
            leaf_vec.drain_min_pair();
        
            let joint_leaf = HuffLeaf::new(None, min_pair[0].get_frequency() + min_pair[1].get_frequency());
            leaf_vec.push(joint_leaf);
                
            self.add(joint_leaf, min_pair)
        }
    }
}

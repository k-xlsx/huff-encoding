#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};



#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HuffLeaf{
    character: Option<char>,
    frequency: u32,
    seed: u128,
    id: u64
}

impl HuffLeaf{
    pub fn new(character: Option<char>, frequency: u32) -> HuffLeaf{
    
        let mut leaf = HuffLeaf{
            character: character, 
            frequency: frequency, 
            seed: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_micros(),
            id: 0
        };

        leaf.id = leaf.calc_id();

        return leaf
    }

    pub fn get_character(&self) -> Option<char>{
        return self.character
    }

    pub fn get_frequency(&self) -> u32{
        return self.frequency
    }

    pub fn get_id(&self) -> u64{
        return self.id
    }


    fn calc_id(&mut self) -> u64{
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        return s.finish()
    }
}



#[derive(Debug)]
pub struct HuffLeafVec{
    vec: Vec<HuffLeaf> ,
    min_pair: Option<[HuffLeaf; 2]>
}

impl HuffLeafVec{
    pub fn new(chars_to_freq: &HashMap<char, u32>) -> HuffLeafVec{
        let mut leaf_vec = HuffLeafVec{
            vec: Vec::new(),
            min_pair: None
        };

        leaf_vec.build_vec(chars_to_freq);
        leaf_vec.set_min_pair(true);

        return leaf_vec;
    }

    pub fn push(&mut self, leaf: HuffLeaf){
        self.vec.push(leaf);
        
        self.set_min_pair(true);
    }

    pub fn drain_min_pair(&mut self){
        if self.min_pair != None{
            self.vec.drain(0..2);
        }

        self.set_min_pair(false);
    }

    pub fn get_min_pair(&self) -> Option<[HuffLeaf; 2]>{
        return self.min_pair;
    }

    pub fn len(&self) -> usize{
        return self.vec.len()
    }


    fn build_vec(&mut self, chars_to_freq: &HashMap<char, u32>){
        for (c, f) in chars_to_freq{
            let new_leaf = HuffLeaf::new(Some(*c), *f);

            self.push(new_leaf);
        }
    }

    fn set_min_pair(&mut self, sort_vec: bool){
        if sort_vec{
            self.sort_by_freq();
        }

        if self.vec.len() <= 1{
            self.min_pair = None;
        }
        else{
            self.min_pair = Some([self.vec[0], self.vec[1]]);
        }

    }

    fn sort_by_freq(&mut self){
        self.vec.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        self.vec.reverse();
    }
}



#[derive(Debug)]
pub struct HuffTree{
    tree: HashMap<HuffLeaf, [HuffLeaf; 2]>,
    root: Option<HuffLeaf>
}

impl HuffTree{
    pub fn new() -> HuffTree{
        return HuffTree{
            tree: HashMap::new(),
            root: None
        }
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

}

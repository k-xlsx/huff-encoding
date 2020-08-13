#![allow(dead_code)]


use std::collections::HashMap;
use crate::huff_structs::{HuffBranch, HuffLeaf};



pub struct HuffBranchVec{
    vec: Vec<HuffBranch>,
}

impl HuffBranchVec{
    pub fn from(chars_to_freq: &HashMap<char, u32>) -> HuffBranchVec{
        let mut leaf_vec = HuffBranchVec::new();

        leaf_vec.build(chars_to_freq);
        leaf_vec.sort();

        return leaf_vec;
    }

    pub fn new() -> HuffBranchVec{
        let leaf_vec = HuffBranchVec{
            vec: Vec::new(),
        };

        return leaf_vec;
    }



    pub fn len(&self) -> usize{
        return self.vec.len();
    }


    pub fn push(&mut self, branch: HuffBranch){
        self.vec.push(branch);
        self.sort();
    }

    pub fn pop_min(&mut self) -> HuffBranch{
        return self.vec.pop().unwrap();
    }


    fn build(&mut self, chars_to_freq: &HashMap<char, u32>){
        for (c, f) in chars_to_freq{
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(*c), *f), [None, None]);
    
            self.push(new_branch);
        }
    }

    fn sort(&mut self){
        self.vec.sort_by(|a, b| b.leaf().frequency().cmp(&a.leaf().frequency()));
    }
}

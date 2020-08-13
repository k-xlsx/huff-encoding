#![allow(dead_code)]


use std::collections::HashMap;
use std::collections::BinaryHeap;
use crate::huff_structs::{HuffBranch, HuffLeaf};



pub struct HuffBranchHeap{
    heap: BinaryHeap<HuffBranch>,
}

impl HuffBranchHeap{
    pub fn from(chars_to_freq: &HashMap<char, u32>) -> HuffBranchHeap{
        let mut leaf_vec = HuffBranchHeap::new();

        leaf_vec.build(chars_to_freq);

        return leaf_vec;
    }

    pub fn new() -> HuffBranchHeap{
        let leaf_vec = HuffBranchHeap{
            heap: BinaryHeap::new(),
        };

        return leaf_vec;
    }



    pub fn len(&self) -> usize{
        return self.heap.len();
    }


    pub fn push(&mut self, branch: HuffBranch){
        self.heap.push(branch);
    }

    pub fn pop_min(&mut self) -> HuffBranch{
        return self.heap.pop().unwrap();
    }


    fn build(&mut self, chars_to_freq: &HashMap<char, u32>){
        for (c, f) in chars_to_freq{
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(*c), *f), [None, None]);
    
            self.push(new_branch);
        }
    }
}

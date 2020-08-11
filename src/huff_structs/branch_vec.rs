use std::collections::HashMap;
use std::rc::Rc;
use crate::huff_structs::{HuffBranch, HuffLeaf};


#[derive(Debug)]
pub struct HuffBranchVec{
    vec: Vec<Rc<HuffBranch>>,
}

impl HuffBranchVec{
    pub fn from(chars_to_freq: &HashMap<char, u32>) -> HuffBranchVec{
        let mut leaf_vec = HuffBranchVec::new();

        leaf_vec.build(chars_to_freq);
        leaf_vec.sort_by_freq();

        return leaf_vec;
    }

    pub fn new() -> HuffBranchVec{
        let leaf_vec = HuffBranchVec{
            vec: Vec::new(),
        };

        return leaf_vec;
    }


    pub fn min(&self) -> &Rc<HuffBranch>{
        return &self.vec[0];
    }

    pub fn min_next(&self) -> &Rc<HuffBranch>{
        return &self.vec[1];
    }

    pub fn len(&self) -> usize{
        return self.vec.len();
    }


    pub fn push(&mut self, branch: Rc<HuffBranch>){
        self.vec.push(branch);
        self.sort_by_freq();
    }

    pub fn drain_min_pair(&mut self){
        self.vec.drain(0..2);
    }


    fn build(&mut self, chars_to_freq: &HashMap<char, u32>){
        for (c, f) in chars_to_freq{
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(*c), *f), None, None, None, None);
    
            self.push(Rc::new(new_branch));
        }
    }

    fn sort_by_freq(&mut self){
        self.vec.sort_by(|a, b| b.leaf().frequency().cmp(&a.leaf().frequency()));
        self.vec.reverse();
    }
}
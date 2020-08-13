#![allow(dead_code)]


use std::rc::Rc;
use std::collections::HashMap;
use crate::huff_structs::{HuffBranch, HuffLeaf};
use crate::huff_structs::branch_vec::HuffBranchVec;


#[derive(Debug)]
pub struct HuffTree{
    root: Option<HuffBranch>,
    branches: Vec<HuffBranch>
}

impl HuffTree{
    pub fn new() -> HuffTree{
        let huff_tree = HuffTree{
            root: None,
            branches: Vec::new(),
        };

        return huff_tree;
    }

    pub fn from(chars_to_freq: &HashMap<char, u32>) -> HuffTree{
        let mut huff_tree = HuffTree::new();

        huff_tree.build(chars_to_freq);

        return huff_tree;
    }


    pub fn root(&self) -> Option<&HuffBranch>{
        match self.root{
            Some(_) =>
                return self.root.as_ref(),
            None =>
                return None,
        }
    }

    pub fn branches(&self) -> &Vec<HuffBranch>{
        return &self.branches;
    }


    fn add(&mut self, branch: HuffBranch){
        self.branches.push(branch);
    }

    fn build(&mut self, chars_to_freq: &HashMap<char, u32>){
        let mut branch_vec = HuffBranchVec::from(&chars_to_freq);


        while branch_vec.len() > 1{
            let mut min = branch_vec.pop_min();
            let mut next_min = branch_vec.pop_min();

            min.set_pos_in_parent(0);
            next_min.set_pos_in_parent(1);

            let branch = HuffBranch::new(
                HuffLeaf::new(
                    None,
                    min.leaf().frequency() + next_min.leaf().frequency()
                ),
                [Some(Rc::new(min)), Some(Rc::new(next_min))]
            );

            self.add(branch.clone());
            branch_vec.push(branch);
        }

        self.root = Some(branch_vec.pop_min());
    }
}

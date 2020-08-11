#![allow(dead_code)]


use std::rc::Rc;
use std::collections::HashMap;
use crate::huff_structs::{HuffBranch, HuffLeaf};
use crate::huff_structs::branch_vec::HuffBranchVec;


#[derive(Debug)]
pub struct HuffTree{
    root: Option<Rc<HuffBranch>>,
    branches: Vec<Rc<HuffBranch>>
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


    pub fn root(&self) -> Option<&Rc<HuffBranch>>{
        match self.root{
            Some(_) =>
                return self.root.as_ref(),
            None =>
                return None,
        }
    }

    pub fn branches(&self) -> &Vec<Rc<HuffBranch>>{
        return &self.branches;
    }


    fn add(&mut self, branch: Rc<HuffBranch>){
        self.branches.push(branch);
    }

    fn build(&mut self, chars_to_freq: &HashMap<char, u32>){
        let mut branch_vec = HuffBranchVec::from(&chars_to_freq);


        while branch_vec.len() > 1{
            let min_pair = branch_vec.min_pair();

            let branch_children = [Some(min_pair.0.clone()), Some(min_pair.1.clone())];
            let branch = HuffBranch::new(
                HuffLeaf::new(
                    None,
                    min_pair.0.leaf().frequency() + min_pair.1.leaf().frequency()
                ),
                branch_children
            );

            branch_vec.drain_min_pair();

            self.add(Rc::new(branch.clone()));
            branch_vec.push(Rc::new(branch))
        }

        self.root = Some(branch_vec.min().clone());
    }
}
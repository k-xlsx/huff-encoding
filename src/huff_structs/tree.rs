#![allow(dead_code)]


use std::rc::Rc;
use std::collections::HashMap;
use crate::huff_structs::{HuffBranch, HuffLeaf};
use crate::huff_structs::branch_heap::HuffBranchHeap;


#[derive(Debug)]
pub struct HuffTree{
    root: Option<HuffBranch>,
}

impl HuffTree{

    pub fn from(ctf: &HashMap<char, u32>) -> HuffTree{
        let mut huff_tree = HuffTree::new(None);

        huff_tree.grow(ctf);

        return huff_tree;
    }

    pub fn new(root: Option<HuffBranch>) -> HuffTree{
        let huff_tree = HuffTree{
            root: root,
        };

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


    pub fn grow(&mut self, ctf: &HashMap<char, u32>){
        let mut branch_vec = HuffBranchHeap::from(&ctf);


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

            branch_vec.push(branch);
        }

        self.root = Some(branch_vec.pop_min());
    }
}

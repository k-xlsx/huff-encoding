use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use crate::huff_structs::{HuffBranch, HuffLeaf};
use crate::huff_structs::branch_vec::HuffBranchVec;


#[derive(Debug)]
pub struct HuffTree{
    root: Option<Rc<HuffBranch>>,
    branches: HashSet<Rc<HuffBranch>>
}

impl HuffTree{
    pub fn new() -> HuffTree{
        let huff_tree = HuffTree{
            root: None,
            branches: HashSet::new(),
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

    pub fn branches(&self) -> &HashSet<Rc<HuffBranch>>{
        return &self.branches;
    }


    fn add(&mut self, branch: Rc<HuffBranch>){
        let r = self.branches.insert(branch);
        println!("{}", r)
    }

    fn build(&mut self, chars_to_freq: &HashMap<char, u32>){
        let mut branch_vec = HuffBranchVec::from(&chars_to_freq);

        while branch_vec.len() > 1{
            let min = branch_vec.min();
            let min_next = branch_vec.min_next();

            let branch = HuffBranch::new(
                HuffLeaf::new(
                    None,
                    min.leaf().frequency() + min_next.leaf().frequency()
                ),
                None,
                None,
                Some(min.clone()),
                Some(min_next.clone()),
            );
            branch_vec.drain_min_pair();
            self.add(Rc::new(branch.clone()));
            self.root = Some(Rc::new(branch.clone()));
            branch_vec.push(Rc::new(branch))

            
        }
    }
}
use bitvec::prelude::*;

use std::{
    cell::RefCell, 
    cmp::Ordering,
};


use super::{HuffLeaf, HuffLetter};



#[derive(Debug, Clone, Eq)]
pub struct HuffBranch<L: HuffLetter>{
    leaf: HuffLeaf<L>,

    pos_in_parent: Option<u8>,
    children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>
}

impl<L: HuffLetter> Ord for HuffBranch<L>{
    fn cmp(&self, other: &Self) -> Ordering {
        self.leaf().cmp(other.leaf())
    }
}

impl<L: HuffLetter> PartialOrd for HuffBranch<L>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: HuffLetter> PartialEq for HuffBranch<L>{
    fn eq(&self, other: &Self) -> bool {
        self.leaf() == other.leaf()
    }
}

impl<L: HuffLetter> HuffBranch<L>{
    pub fn new(leaf: HuffLeaf<L>, children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>) -> Self{
        HuffBranch{
            leaf,

            pos_in_parent: None,
            children
        }
    }

    pub fn leaf(&self) -> &HuffLeaf<L>{
        &self.leaf
    }

    pub fn pos_in_parent(&self) -> Option<u8>{
        self.pos_in_parent
    }

    pub fn children(&self) -> Option<&[Box<RefCell<HuffBranch<L>>>; 2]>{
        match self.children{
            None => 
                None,
            Some(_) => {
                self.children.as_ref()
            }
        }
    }


    pub fn set_children(&mut self, children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>){
        self.children = children;
    }

    pub fn set_pos_in_parent(&mut self, pos_in_parent: u8){
        self.pos_in_parent = Some(pos_in_parent);
    } 

    pub fn set_code(&mut self, parent_code: Option<&BitVec<Msb0, u8>>){
        let mut code = BitVec::new();

        if let Some(pos_in_parent) = self.pos_in_parent(){
            if let Some(parent_code) = parent_code{
                for bit in parent_code{
                    code.push(*bit);
                }
            }
            
            code.push(pos_in_parent >= 1);

            self.leaf.set_code(code);
        }
    }
}

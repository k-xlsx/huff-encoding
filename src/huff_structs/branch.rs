#![allow(dead_code)]


use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use bit_vec::BitVec;
use crate::huff_structs::HuffLeaf;



/// Struct representing a node in the Huffman Tree.
/// 
/// Stores its children as:
/// ```
/// [Option<Rc<RefCell<HuffBranch>>>; 2]
/// ```
/// Also stores its position in the parent's children Array, and 
/// data represented as a HuffLeaf.
#[derive(Debug, Clone, Eq)]
pub struct HuffBranch{
    leaf: HuffLeaf,

    pos_in_parent: Option<u8>,
    children: Option<[Rc<RefCell<HuffBranch>>; 2]>
}

impl Ord for HuffBranch {
    fn cmp(&self, other: &Self) -> Ordering {
        other.leaf().frequency().cmp(&self.leaf().frequency())
    }
}

impl PartialOrd for HuffBranch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffBranch {
    fn eq(&self, other: &Self) -> bool {
        self.leaf().frequency() == other.leaf().frequency()
    }
}

impl HuffBranch{
    pub fn new(leaf: HuffLeaf, children: Option<[Rc<RefCell<HuffBranch>>; 2]>) -> HuffBranch{
        //! Initializes a new HuffBranch.
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::{HuffBranch, HuffLeaf};
        //! 
        //! let foo = HuffBranch::new(HuffLeaf::new('s', 3), [None, None]);
        //! ```


        let huff_branch = HuffBranch{
            leaf: leaf,

            pos_in_parent: None,
            children: children
        };

        return huff_branch;
    }


    pub fn leaf(&self) -> &HuffLeaf{
        //! Returns a reference to the stored HuffLeaf.


        return &self.leaf;
    }

    pub fn pos_in_parent(&self) -> Option<u8>{
        //! Returns its position in the parent's children Array


        return self.pos_in_parent
    }

    pub fn children(&self) -> Option<&[Rc<RefCell<HuffBranch>>; 2]>{
        //! Returns the stored Array of the branch's children

        match self.children{
            None => 
                return None,
            Some(_) => {
                return self.children.as_ref();
            }
        }
    }


    pub fn set_children(&mut self, children: Option<[Rc<RefCell<HuffBranch>>; 2]>){
        //! Sets the given children array

        self.children = children;
    }

    pub fn set_pos_in_parent(&mut self, pos_in_parent: u8){
        //! Sets the stored position in parent's children Array


        self.pos_in_parent = Some(pos_in_parent);
    } 

    pub fn set_code(&mut self, parent_code: Option<&BitVec>){
        //! Sets its leaf's code based on the given parent_code and its
        //! pos_in_parent.


        let mut code = BitVec::new();

        match self.pos_in_parent(){
            Some(_) =>{
                match parent_code{
                    Some(_) =>{
                        for bit in parent_code.unwrap(){
                            code.push(bit);
                        }
                    }
                    None =>
                        (),
                }
                
                code.push(self.pos_in_parent().unwrap() >= 1);

                self.leaf.set_code(code);
            }
            None =>
                (),
        }
    }
}

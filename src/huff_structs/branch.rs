#![allow(dead_code)]


use std::rc::Rc;
use std::cmp::Ordering;
use crate::huff_structs::HuffLeaf;


/// Struct representing a node in the Huffman Tree.
/// 
/// Stores its children as:
/// ```
/// [Option<Rc<HuffBranch>>; 2]
/// ```
/// Also stores its position in the parent's children Array, and 
/// data represented as a HuffLeaf.
#[derive(Debug, Clone, Eq)]
pub struct HuffBranch{
    leaf: HuffLeaf,

    pos_in_parent: Option<u8>,
    children: [Option<Rc<HuffBranch>>; 2]
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
    /// Initializes a new HuffBranch.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::huff_structs::{HuffBranch, HuffLeaf};
    /// 
    /// let hb = HuffBranch::new(HuffLeaf::new('s', 3), [None, None]);
    /// ```
    pub fn new(leaf: HuffLeaf, children: [Option<Rc<HuffBranch>>; 2]) -> HuffBranch{

        let huff_branch = HuffBranch{
            leaf: leaf,

            pos_in_parent: None,
            children: children
        };

        return huff_branch;
    }


    /// Returns a reference to the stored HuffLeaf.
    pub fn leaf(&self) -> &HuffLeaf{
        return &self.leaf;
    }

    /// Returns its position in the parent's children Array
    pub fn pos_in_parent(&self) -> Option<u8>{
        return self.pos_in_parent
    }

    /// Returns the stored Array of the branch's children
    pub fn children(&self) -> [Option<&Rc<HuffBranch>>; 2]{
        return [self.children[0].as_ref(), self.children[1].as_ref()]
    }

    /// Sets the stored position in parent's children Array
    pub fn set_pos_in_parent(&mut self, pos_in_parent: u8){
        self.pos_in_parent = Some(pos_in_parent);
    } 

    /// Sets its leaf's code based on the give parent_code and its
    /// pos_in_parent.
    pub fn set_leaf_code(&mut self, parent_code: Option<&String>){
        let mut code = String::new();

        match self.pos_in_parent(){
            Some(_) =>{        
                println!("has parent");
                match parent_code{
                    Some(_) =>{
                        code.push_str(&parent_code.unwrap().chars().rev().collect::<String>());
                        println!("parent has parent, code: {}", code);
                    }
                    None =>
                        println!("parent is root")
                }
                match self.pos_in_parent().unwrap(){
                    0 =>
                        code.push('0'),
                    1 =>
                        code.push('1'),
                    _ =>
                        panic!("pos_in_parent not binary"),
                }

                code = code.chars().rev().collect::<String>();
                self.leaf.set_code(&code);
                println!("{}", code);
            }
            None =>
                println!("no parent")
        }
    }
}

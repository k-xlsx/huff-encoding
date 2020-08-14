#![allow(dead_code)]


use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use crate::huff_structs::{HuffBranch, HuffLeaf, get_chars_to_freq};
use crate::huff_structs::branch_heap::HuffBranchHeap;



/// Struct representing a Huffman Tree.
/// 
/// A HuffTree is comprised of HuffBranches, each having
/// 2 or 0 children, with root being the top one and 
/// every bottom one containing a char.
/// 
/// Can be grown from: 
/// ```
/// HashMap<char, u32>
/// ```
/// or 
/// ```
/// &str
/// ```
/// or even initialized empty and grown afterwards.
/// 
#[derive(Debug)]
pub struct HuffTree{
    root: Option<HuffBranch>,
}

impl HuffTree{
    /// Creates a HuffTree from:
    /// ```
    /// &str
    /// ```
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::huff_structs::HuffTree;
    /// 
    /// let ht = HuffTree::from("Hello, World!");
    /// ```
    pub fn from(s: &str) -> HuffTree{
        let mut huff_tree = HuffTree::new(None);
        huff_tree.grow(s);

        return huff_tree
    } 

    /// Creates a HuffTree from:
    /// ```
    /// HashMap<char, u32>
    /// ```
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::huff_structs::{HuffTree, get_chars_to_freq};
    /// 
    /// let ht = HuffTree::from(get_chars_to_freq("Hello, World!"));
    /// ```
    pub fn from_ctf(ctf: &HashMap<char, u32>) -> HuffTree{
        let mut huff_tree = HuffTree::new(None);
        huff_tree.grow_ctf(ctf);

        return huff_tree;
    }

    /// Initializes a HuffTree with the given root.
    /// 
    /// Can be grown later with .grow or .grow_ctf
    /// 
    /// # Example
    /// ```
    /// use huff_encoding::huff_structs::HuffTree;
    /// 
    /// let ht = HuffTree::new();
    /// ht.grow("Hello, World!");
    /// ```
    pub fn new(root: Option<HuffBranch>) -> HuffTree{
        let huff_tree = HuffTree{
            root: root,
        };

        return huff_tree;
    }

    
    /// Returns the root of the tree.
    pub fn root(&self) -> Option<&HuffBranch>{
        match self.root{
            Some(_) =>
                return self.root.as_ref(),
            None =>
                return None,
        }
    }


    /// Grows the tree from the given:HuffTree
    /// ```
    /// &str
    /// ```
    pub fn grow(&mut self, s: &str){
        assert!(s.len() > 0, "slice is empty");
        self.grow_ctf(&get_chars_to_freq(s));
    }

    /// Grows the tree from the given:HuffTree
    /// ```
    /// &HashMap<char, u32>
    /// ```
    pub fn grow_ctf(&mut self, ctf: &HashMap<char, u32>){
        assert!(ctf.len() > 0, "ctf is empty");

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
                [Some(Rc::new(RefCell::new(min))), Some(Rc::new(RefCell::new(next_min)))]
            );

            branch_vec.push(branch);
        }

        self.root = Some(branch_vec.pop_min());

        HuffTree::set_codes(RefCell::new(self.root.clone().unwrap()).borrow_mut());
    }


    fn set_codes(root: RefMut<HuffBranch>){
        let root = root;
        let children = root.children();

        match children{
            [Some(_), Some(_)] =>{
                let root_code = root.leaf().code();
                for child in children.iter(){
                    child.unwrap().borrow_mut().set_code(root_code);
                    HuffTree::set_codes(child.unwrap().borrow_mut());
                }
            }
            _ =>
                (),
        }
    }
}

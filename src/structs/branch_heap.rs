use std::collections::HashMap;
use std::collections::BinaryHeap;
use crate::structs::{HuffBranch, HuffLeaf};



/// A BinaryHeap of HuffBranches, ordered by frequency(greatest to lowest), 
/// used to grow the HuffTree.
/// 
/// Can be either initialized empty, or made from:
/// ```
/// HashMap<char, usize>
/// ```
pub struct HuffBranchHeap{
    heap: BinaryHeap<HuffBranch>,
}

impl HuffBranchHeap{
    pub fn from(cfg: &HashMap<char, usize>) -> HuffBranchHeap{
        //! Creates a HuffBranchHeap from a HashMap of chars as
        //! keys and their frequencies(usize) as values.
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::{HuffBranchHeap, chars_to_freq}
        //! 
        //! let foo = HuffBranchHeap::from(get_chars_to_freq("Hello, World!"));
        //! ```


        let mut leaf_vec = HuffBranchHeap::new();

        leaf_vec.build(cfg);

        return leaf_vec;
    }

    pub fn new() -> HuffBranchHeap{
        //! Initializes an empty HuffBranchHeap.
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffBranchHeap
        //! 
        //! let foo = HuffBranchHeap::new();
        //! ```


        let leaf_vec = HuffBranchHeap{
            heap: BinaryHeap::new(),
        };

        return leaf_vec;
    }


    pub fn len(&self) -> usize{
        //! Returns the length of the heap.


        return self.heap.len();
    }

    
    pub fn push(&mut self, branch: HuffBranch){
        //! Pushes the branch onto the heap.


        self.heap.push(branch);
    }

    pub fn pop_min(&mut self) -> HuffBranch{
        //! Pops a branch (which is always the lowest frequency one) of the heap.


        return self.heap.pop().unwrap();
    }


    fn build(&mut self, cfg: &HashMap<char, usize>){
        for (c, f) in cfg{
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(*c), *f), None);
    
            self.push(new_branch);
        }
    }
}

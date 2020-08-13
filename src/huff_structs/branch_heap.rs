#![allow(dead_code)]


use std::collections::HashMap;
use std::collections::BinaryHeap;
use crate::huff_structs::{HuffBranch, HuffLeaf};


/// A BinaryHeap of HuffBranches, ordered by frequency(greatest to lowest), 
/// used to grow the HuffTree.
/// 
/// Can be either initialized empty, or made from:
/// ```
/// HashMap<char, u32>
/// ```
pub struct HuffBranchHeap{
    heap: BinaryHeap<HuffBranch>,
}

impl HuffBranchHeap{
    /// Creates a HuffBranchHeap from a HashMap of chars as
    /// keys and their frequencies(u32) as values.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::huff_structs::{HuffBranchHeap, chars_to_freq}
    /// 
    /// let hbh = HuffBranchHeap::from(get_chars_to_freq("Hello, World!"));
    /// ```
    pub fn from(cfg: &HashMap<char, u32>) -> HuffBranchHeap{
        let mut leaf_vec = HuffBranchHeap::new();

        leaf_vec.build(cfg);

        return leaf_vec;
    }

    /// Initializes an empty HuffBranchHeap.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::huff_structs::HuffBranchHeap
    /// 
    /// let hbh = HuffBranchHeap::new();
    /// ```
    pub fn new() -> HuffBranchHeap{
        let leaf_vec = HuffBranchHeap{
            heap: BinaryHeap::new(),
        };

        return leaf_vec;
    }


    /// Returns the length of the heap.
    pub fn len(&self) -> usize{
        return self.heap.len();
    }

    /// Pushes the branch onto the heap.
    pub fn push(&mut self, branch: HuffBranch){
        self.heap.push(branch);
    }

    /// Pops a branch (which is always the lowest frequency one) of the heap.
    pub fn pop_min(&mut self) -> HuffBranch{
        return self.heap.pop().unwrap();
    }


    fn build(&mut self, cfg: &HashMap<char, u32>){
        for (c, f) in cfg{
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(*c), *f), [None, None]);
    
            self.push(new_branch);
        }
    }
}

use std::collections::BinaryHeap;

use crate::{HuffBranch, HuffLeaf, ByteFreqs};



/// A BinaryHeap of HuffBranches, ordered by frequency (greatest to lowest), 
/// used to grow the HuffTree.
/// 
/// Can be either initialized empty, or made from:
/// ```
/// HashMap<u8, usize>
/// ```
pub struct HuffBranchHeap{
    heap: BinaryHeap<HuffBranch>,
}

impl HuffBranchHeap{
    /// Creates a HuffBranchHeap from a HashMap of u8 as
    /// keys and their frequencies (usize) as values.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::{HuffBranchHeap, get_byte_freqs}
    /// 
    /// let foo = HuffBranchHeap::from(get_byte_freqs("Hello, World/".as_bytes()));
    /// ```
    pub fn from(byte_freqs: &ByteFreqs) -> HuffBranchHeap{
        let mut leaf_vec = HuffBranchHeap::new();

        leaf_vec.build(byte_freqs);

        return leaf_vec;
    }

    /// Initializes an empty HuffBranchHeap.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::HuffBranchHeap
    /// 
    /// let foo = HuffBranchHeap::new();
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


    fn build(&mut self, byte_freqs: &ByteFreqs){
        for (b, f) in byte_freqs.iter(){
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(*b), *f), None);
    
            self.push(new_branch);
        }
    }
}

use std::collections::BinaryHeap;

use crate::{HuffBranch, HuffLeaf, ByteFreqs};



/// A BinaryHeap of HuffBranches, ordered by frequency (greatest to lowest), 
/// used to grow the HuffTree.
/// 
/// Can be either initialized empty, or made from:
/// 
/// HashMap<u8, usize>
#[derive(Debug)]
pub struct HuffBranchHeap{
    heap: BinaryHeap<HuffBranch>,
}

impl HuffBranchHeap{
    /// Creates a HuffBranchHeap from a HashMap of u8 as
    /// keys and their frequencies (usize) as values.
    pub fn from_byte_freqs(byte_freqs: &ByteFreqs) -> HuffBranchHeap{
        let mut leaf_vec = HuffBranchHeap::new();

        leaf_vec.build(byte_freqs);

        return leaf_vec;
    }

    /// Initializes an empty HuffBranchHeap.
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
        for (b, f) in byte_freqs{
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(b as u8), f), None);
    
            self.push(new_branch);
        }
    }
}

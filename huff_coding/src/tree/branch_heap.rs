use std::{
    collections::BinaryHeap,
    cmp::Ordering,
};

use super::{HuffBranch, HuffLeaf, HuffLetter};
use crate::freqs::Freq;



/// A struct used to build a ```HuffTree```
/// 
/// Stores ```HuffBranch```es inside a ```std::collections::BinaryHeap```, but reversed
/// (branches with the smallest frequencies are at the end, so they can be easily popped)
#[derive(Debug, Clone)]
pub struct HuffBranchHeap<L: HuffLetter>{
    heap: BinaryHeap<HuffBranchHeapItem<L>>,
}

impl<L: HuffLetter> HuffBranchHeap<L>{
    /// Initialize a new ```HuffBranchHeap``` from the given freqs struct
    pub fn from_freq<F: Freq<L>>(freqs: F) -> Self{
        let mut heap = HuffBranchHeap::new();
        heap.build(freqs);
        heap
    }

    /// Initialize an empty ```HuffBranchHeap```
    pub fn new() -> Self{
        HuffBranchHeap::<L>{
            heap: BinaryHeap::new(),
        }
    }

    /// Return the lenght of the heap
    pub fn len(&self) -> usize{
        self.heap.len()
    }

    /// Push a branch onto the heap
    pub fn push(&mut self, branch: HuffBranch<L>){
        self.heap.push(HuffBranchHeapItem(branch));
    }

    /// Pop the ```HuffBranch``` with the smallest frequency
    pub fn pop_min(&mut self) -> HuffBranch<L>{
        self.heap.pop().unwrap().unwrap()
    }

    fn build<F: Freq<L>>(&mut self, freqs: F){
        for (l, f) in freqs.into_iter(){
            let new_branch = HuffBranch::new(HuffLeaf::new(Some(l), f), None);
    
            self.push(new_branch);
        }
    }
}

/// A wrapper for HuffBranch that reverses it's cmp
/// for the smallest HuffBranches to appear at the end of the 
/// HuffBranchHeap
#[derive(Debug, Clone, Eq)]
struct HuffBranchHeapItem<L: HuffLetter>(HuffBranch<L>);

impl<L: HuffLetter> Ord for HuffBranchHeapItem<L>{
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.leaf().cmp(&self.0.leaf())
    }
}

impl<L: HuffLetter> PartialOrd for HuffBranchHeapItem<L>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: HuffLetter> PartialEq for HuffBranchHeapItem<L>{
    fn eq(&self, other: &Self) -> bool {
        self.0.leaf().frequency() == other.0.leaf().frequency()
    }
}

impl<L: HuffLetter> HuffBranchHeapItem<L>{
    /// Consumes self and returns the wrapped branch
    pub fn unwrap(self) -> HuffBranch<L>{
        self.0
    }
}

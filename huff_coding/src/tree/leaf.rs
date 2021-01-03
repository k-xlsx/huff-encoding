use super::HuffLetter;
use crate::bitvec::prelude::*;

use std::cmp::Ordering;



/// Struct representing a HuffBranch's data.
/// 
/// Stores:
/// * `letter: Option<L>`
///  * type implementing `HuffLetter`
///  * if is a joint branch then `letter == None`
/// * `weight: usize`
/// * `code: Option<BitVec<Msb0, u8>>` (big endian)
/// 
/// *Can be compared with an another `HuffLeaf` by their weights*
#[derive(Debug, Eq, Clone)]
pub struct HuffLeaf<L: HuffLetter>{
    letter: Option<L>,
    weight: usize,
    code: Option<BitVec<Msb0, u8>>,
}

impl<L: HuffLetter> Ord for HuffLeaf<L> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<L: HuffLetter> PartialOrd for HuffLeaf<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: HuffLetter> PartialEq for HuffLeaf<L>{
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<L: HuffLetter> HuffLeaf<L>{
    /// Initialize a HuffLeaf with the given letter and weight 
    /// (code is at first set to None and can be changed with the `set_code` method)
    pub fn new(letter: Option<L>, weight: usize) -> Self{
        HuffLeaf{
            letter,
            weight,
            code: None,
        }
    }

    /// Returns a reference to the stored letter 
    pub fn letter(&self) -> Option<&L>{
        self.letter.as_ref()
    }

    /// Returns the stored weight
    pub fn weight(&self) -> usize{
        self.weight
    }

    /// Returns the stored code
    pub fn code(&self) -> Option<&BitVec<Msb0, u8>>{
        self.code.as_ref()
    }
    
    /// Set the given code, consuming the original
    pub fn set_code(&mut self, code: BitVec<Msb0, u8>){    
        self.code = Some(code);
    }
}

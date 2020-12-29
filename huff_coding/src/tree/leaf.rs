use super::HuffLetter;
use crate::bitvec::prelude::*;

use std::cmp::Ordering;



/// Struct representing a HuffBranch's data.
/// 
/// Stores:
/// * ```letter: Option<L>```
///  * type implementing ```HuffLetter```
///  * if is a joint branch then ```letter == None```
/// * ```frequency: usize```
/// * ```code: Option<BitVec<Msb0, u8>>``` (big endian)
/// 
/// *Can be compared with an another ```HuffLeaf``` by their frequencies*
#[derive(Debug, Eq, Clone)]
pub struct HuffLeaf<L: HuffLetter>{
    letter: Option<L>,
    frequency: usize,
    code: Option<BitVec<Msb0, u8>>,
}

impl<L: HuffLetter> Ord for HuffLeaf<L> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency().cmp(&other.frequency())
    }
}

impl<L: HuffLetter> PartialOrd for HuffLeaf<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: HuffLetter> PartialEq for HuffLeaf<L>{
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl<L: HuffLetter> HuffLeaf<L>{
    /// Initialize a HuffLeaf with the given letter and frequency 
    /// (code is at first set to None and can be changed with the ```set_code``` method)
    pub fn new(letter: Option<L>, frequency: usize) -> Self{
        HuffLeaf{
            letter,
            frequency,
            code: None,
        }
    }

    /// Returns a reference to the stored letter 
    pub fn letter(&self) -> Option<&L>{
        self.letter.as_ref()
    }

    /// Returns the stored frequency
    pub fn frequency(&self) -> usize{
        self.frequency
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

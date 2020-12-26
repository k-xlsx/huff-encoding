
use bitvec::prelude::*;

use std::cmp::Ordering;

use super::HuffLetter;



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
    pub fn new(letter: Option<L>, frequency: usize) -> Self{
        HuffLeaf{
            letter,
            frequency,
            code: None,
        }
    }

    pub fn letter(&self) -> Option<&L>{
        self.letter.as_ref()
    }

    pub fn frequency(&self) -> usize{
        self.frequency
    }

    pub fn code(&self) -> Option<&BitVec<Msb0, u8>>{
        self.code.as_ref()
    }
    
    pub fn set_code(&mut self, code: BitVec<Msb0, u8>){    
        self.code = Some(code);
    }
}

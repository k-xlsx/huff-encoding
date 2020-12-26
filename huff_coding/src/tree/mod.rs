mod leaf;
mod branch;
mod branch_heap;
mod tree;


pub use self::{
    leaf::HuffLeaf, 
    branch::HuffBranch, 
    tree::HuffTree,
};



pub trait HuffLetter: Clone + std::hash::Hash + Eq{}
pub trait HuffLetterBitStore: HuffLetter + Copy + bitvec::prelude::BitStore{}

impl HuffLetter for u8{}
impl HuffLetter for u16{}
impl HuffLetter for u32{}
impl HuffLetter for u64{}
impl HuffLetterBitStore for u8{}
impl HuffLetterBitStore for u16{}
impl HuffLetterBitStore for u32{}
impl HuffLetterBitStore for u64{}
impl HuffLetter for u128{}
impl HuffLetter for i8{}
impl HuffLetter for i16{}
impl HuffLetter for i32{}
impl HuffLetter for i64{}
impl HuffLetter for i128{}
impl HuffLetter for char{}
impl HuffLetter for bool{}
impl HuffLetter for &str{}
impl HuffLetter for String{}

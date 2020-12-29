mod leaf;
mod branch;
mod branch_heap;
mod tree;
mod bits;


pub use self::{
    leaf::HuffLeaf, 
    branch::HuffBranch, 
    tree::HuffTree,
    bits::{HuffCode, HuffTreeBin},
};


use std::{
    hash::Hash,
    mem::size_of,
    convert::TryInto,
};



/// Trait specifying that the given type can be stored in a ```HuffTree```, which means
/// it implements:
/// 
/// ```Clone``` + ```Eq``` + ```std::hash::Hash```
/// 
/// Implemented by default for every primitive type, as well as String.
/// 
pub trait HuffLetter: Clone + Eq + Hash{}
/// Trait specifying that the given HuffLetter can be converted
/// into bytes *(returns ```Vec<u8>``` 'cause i have no idea right now)* and
/// can be created from bytes (```&[u8]```)
/// 
/// so the ```HuffTree``` can be represented in binary.
/// 
/// Implemented by default for every integer
pub trait HuffLetterAsBytes: HuffLetter{
    fn try_from_be_bytes(bytes: &[u8]) ->  Result<Self, Box<dyn std::error::Error>>;
    fn to_be_byte_vec(&self) -> Vec<u8>;
}


/// Implements HuffLetter for every provided type (without generics) 
macro_rules! primitive_letter_impl{
    {$($type:ty),+} => {
        $(
        impl HuffLetter for $type{}
        )+
    };
}
primitive_letter_impl!{
    char,
    &str, 
    String
}

/// Implements HuffLetter and HuffLetterAsBytes with a default implementation
/// for provided primitive integer types
macro_rules! integer_letter_impl{
    {$($type:ty),+} => {
        $(
        primitive_letter_impl!{$type}
        impl HuffLetterAsBytes for $type{
            fn try_from_be_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>{
                let bytes: [u8; size_of::<$type>()] = bytes.try_into()?;
                Ok(Self::from_be_bytes(bytes))
            }
            fn to_be_byte_vec(&self) -> Vec<u8>{
                self.to_be_bytes().to_vec()
            }
        }
        )+
    };
}
integer_letter_impl!{
    u8, u16, u32, u64, usize, u128, 
    i8, i16, i32, i64, isize, i128
}

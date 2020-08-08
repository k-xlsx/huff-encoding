#![allow(dead_code)]


use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};



#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HuffLeaf{
    character: Option<char>,
    frequency: u32,
    id: u128
}

impl HuffLeaf{
    pub fn new(character: Option<char>, frequency: u32) -> HuffLeaf{
    
        let mut leaf = HuffLeaf{
            character: character, 
            frequency: frequency, 
            id: 0
        };

        leaf.id = HuffLeaf::calc_id();

        return leaf
    }

    pub fn get_character(&self) -> Option<char>{
        return self.character
    }

    pub fn get_frequency(&self) -> u32{
        return self.frequency
    }

    pub fn get_id(&self) -> u128{
        return self.id
    }


    fn calc_id() -> u128{
        return SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_micros();
    }
}
use crate::huff_structs::leaf::HuffLeaf;
use std::collections::HashMap;



#[derive(Debug)]
pub struct HuffLeafVec{
    vec: Vec<HuffLeaf> ,
    min_pair: Option<[HuffLeaf; 2]>
}

impl HuffLeafVec{
    pub fn new(chars_to_freq: &HashMap<char, u32>) -> HuffLeafVec{
        let mut leaf_vec = HuffLeafVec{
            vec: Vec::new(),
            min_pair: None
        };

        leaf_vec.build_vec(chars_to_freq);
        leaf_vec.set_min_pair(true);

        return leaf_vec;
    }

    pub fn push(&mut self, leaf: HuffLeaf){
        self.vec.push(leaf);
        
        self.set_min_pair(true);
    }

    pub fn drain_min_pair(&mut self){
        if self.min_pair != None{
            self.vec.drain(0..2);
        }

        self.set_min_pair(false);
    }

    pub fn get_min_pair(&self) -> Option<[HuffLeaf; 2]>{
        return self.min_pair;
    }

    pub fn len(&self) -> usize{
        return self.vec.len()
    }


    fn build_vec(&mut self, chars_to_freq: &HashMap<char, u32>){
        for (c, f) in chars_to_freq{
            let new_leaf = HuffLeaf::new(Some(*c), *f);

            self.push(new_leaf);
        }
    }

    fn set_min_pair(&mut self, sort_vec: bool){
        if sort_vec{
            self.sort_by_freq();
        }

        if self.vec.len() <= 1{
            self.min_pair = None;
        }
        else{
            self.min_pair = Some([self.vec[0], self.vec[1]]);
        }

    }

    fn sort_by_freq(&mut self){
        self.vec.sort_by(|a, b| b.get_frequency().cmp(&a.get_frequency()));
        self.vec.reverse();
    }
}
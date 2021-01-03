pub use self::byte_weights::ByteWeights;


use std::{
    hash::Hash,
    collections::HashMap,
};



/// Trait signifying that the struct stores the weights of a type `L`, so that
/// for any stored `L` there is a corresponding `usize`(weight).
/// 
/// Implemented by default for `std::collections::HashMap<L, usize>` and
/// for `huff_coding::weights::byte_weights::ByteWeights`
/// 
/// Needed implementations:
/// * Traits:
///  * `Eq`
///  * `Clone`
///  * `IntoIterator<Item = (L, usize)>`
/// * Methods:
///  * `fn get(&self, letter: &L) -> Option<&usize>`
///  * `fn get_mut(&mut self, letter: &L) -> Option<&mut usize>`
///  * `fn len(&self) -> usize`
///  * `fn is_empty(&self) -> bool`
/// 
/// *In order to build with a `huff_coding::tree::HuffTree` `L` must implement `huff_coding::tree::HuffLetter`*
pub trait Weights<L>: Eq + Clone + IntoIterator<Item = (L, usize)>{
    fn get(&self, letter: &L) -> Option<&usize>;
    fn get_mut(&mut self, letter: &L) -> Option<&mut usize>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<L: Eq + Clone + Hash> Weights<L> for HashMap<L, usize>{
    fn get(&self, letter: &L) -> Option<&usize>{
        self.get(letter)
    }
    fn get_mut(&mut self, letter: &L) -> Option<&mut usize>{
        self.get_mut(letter)
    }
    fn len(&self) -> usize{
        self.len()
    }
    fn is_empty(&self) -> bool{
        self.is_empty()
    }
}

/// Module containing the `ByteWeights` struct and stuff related to it
pub mod byte_weights{
    use std::{
        ops::{Add, AddAssign},
        thread,
    };

    use crate::utils::ration_vec;
    use super::Weights;



    // TODO: docs
    #[derive(Clone, Copy, Eq)]
    pub struct ByteWeights{
        weights: [usize; 256],
        len: usize,
    }

    impl PartialEq for ByteWeights{
        fn eq(&self, other: &Self) -> bool {
            self.weights == other.weights
        }
    }

    impl Add for ByteWeights{
        type Output = Self;

        fn add(mut self, other: Self) -> Self {
            self.add_byte_weights(&other);
            self
        }
    }

    impl AddAssign for ByteWeights{
        fn add_assign(&mut self, other: Self){
            self.add_byte_weights(&other);
        }
    }

    impl Weights<u8> for ByteWeights{
        /// Return a reference to the weight corresponding
        /// to the given byte.
        fn get(&self, byte: &u8) -> Option<&usize>{
            let weight = self.weights.get(*byte as usize)?;
            if *weight == 0{
                return None
            }
            Some(weight)
        }

        /// Return a mutable reference to the weight corresponding
        /// to the given byte.
        fn get_mut(&mut self, byte: &u8) -> Option<&mut usize>{
            let weight = self.weights.get_mut(*byte as usize)?;
            if *weight == 0{
                return None
            }
            Some(weight)
        }

        /// Return the number of different counted bytes stored in the `ByteWeights`
        fn len(&self) -> usize{
            self.len
        }

        /// Return true if len == 0
        fn is_empty(&self) -> bool{
            self.len == 0
        }
    }


    // Consuming iterator over the contents (`(u8, usize)`) of `ByteWeights` 
    pub struct IntoIter{
        weights: ByteWeights,
        current_index: usize,
    }

    impl Iterator for IntoIter{
        type Item = (u8, usize);

        fn next(&mut self) -> Option<Self::Item>{
            if self.current_index == 256{
                return None
            }

            while self.weights.get(&(self.current_index as u8)).is_none(){
                if self.current_index == 256{
                    return None
                }
                self.current_index += 1
            }
            let entry = Some((self.current_index as u8, *self.weights.get(&(self.current_index as u8)).unwrap()));
            if self.current_index != 256{self.current_index += 1;}

            entry
        }
    }

    impl IntoIterator for ByteWeights{
        type Item = (u8, usize);
        type IntoIter = IntoIter;

        fn into_iter(self) -> IntoIter{
            IntoIter{weights: self, current_index: 0}
        }   
    }

    /// Non consuming iterator over the contents (`(u8, usize)`) of `ByteWeights` 
    pub struct Iter<'a>{
        weights: &'a ByteWeights,
        current_index: usize,
    }

    impl Iterator for Iter<'_>{
        type Item = (u8, usize);

        fn next(&mut self) -> Option<Self::Item>{
            if self.current_index == 256{
                return None
            }

            while self.weights.get(&(self.current_index as u8)).is_none(){
                if self.current_index == 256{
                    return None
                }
                self.current_index += 1
            }
            let entry = Some((self.current_index as u8, *self.weights.get(&(self.current_index as u8)).unwrap()));
            if self.current_index != 256{self.current_index += 1;}

            entry
        }
    }

    impl <'a> IntoIterator for &'a ByteWeights{
        type Item = (u8, usize);
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Iter<'a>{
            Iter{weights: &self, current_index: 0}
        }   
    }


    impl ByteWeights{
        pub fn from_bytes(bytes: &[u8]) -> Self{
            // count bytes into an array
            let mut weights: [usize; 256] = [0;256];
            let mut len = 0;

            for byte in bytes{
                if weights[*byte as usize] == 0{len += 1;}
                weights[*byte as usize] += 1;
            }
 
            ByteWeights{
                weights,
                len,
            }
        }

        pub fn threaded_from_bytes(bytes: &[u8], thread_num: usize) -> Self{
            // divide the bytes into rations per thread
            let byte_rations = ration_vec(bytes, thread_num);

            // create ByteWeights from every ration
            let mut handles = Vec::with_capacity(thread_num);
            for ration in byte_rations{
                let handle = thread::spawn(move || {
                    ByteWeights::from_bytes(&ration)
                });
                handles.push(handle);
            }

            // push all created ByteWeights into a Vec 
            let mut weights_vec: Vec<ByteWeights> = Vec::with_capacity(thread_num);
            for handle in handles{
                weights_vec.push(handle.join().unwrap());
            }

            // add all ByteWeights into one
            let mut weights = weights_vec.pop().unwrap();
            for weights_other in weights_vec{
                weights += weights_other;
            }

            weights
        }

        /// Returns an iterator over the bytes to their weights `(u8, usize)`
        pub fn iter(&self) -> Iter{
            self.into_iter()
        }

        /// Add another `ByteWeights` to self
        pub fn add_byte_weights(&mut self, other: &ByteWeights){
            for (b, f) in other{
                let self_entry = self.get_mut(&b);
                match self_entry{
                    Some(self_entry) =>{
                        *self_entry += f;
                    }
                    None =>{
                        self.weights[b as usize] = f;
                        self.len += 1;
                    }
                }
            }
        }
    }
}

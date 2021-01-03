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

/// Struct storing the number of occurences of each byte in
/// a provided byte slice.
pub mod byte_weights{
    use std::{
        ops::{Add, AddAssign},
        thread,
    };

    use crate::utils::ration_vec;
    use super::Weights;



    /// Struct storing the number of occurences of each byte in
    /// a provided byte slice.
    /// 
    /// A `HuffTree` can be initialized with it, as `ByteWeights`
    /// implements the `Weights` trait.
    /// 
    /// # Examples
    /// ---
    /// Initialization and interfacing:
    /// ```
    /// use huff_coding::prelude::ByteWeights;
    /// 
    /// let byte_weights = ByteWeights::from_bytes(b"fffff");
    /// assert_eq!(*byte_weights.get(&b'f').unwrap(), 5);
    /// assert_eq!(byte_weights.len(), 1);
    /// ```
    /// Iteration:
    /// ```
    /// use huff_coding::prelude::ByteWeights;
    /// 
    /// let byte_weights = ByteWeights::from_bytes(&[0, 1, 1, 2, 2, 2]);
    /// for (byte, weight) in byte_weights{
    ///     assert_eq!(byte as usize, weight - 1);
    /// }
    /// ```
    /// Adding two `ByteWeights`:
    /// ```
    /// use huff_coding::prelude::ByteWeights;
    /// 
    /// let mut byte_weights = ByteWeights::from_bytes(b"aabbb");
    /// let other = ByteWeights::from_bytes(b"aaabbc");
    /// 
    /// byte_weights += other;
    /// 
    /// assert_eq!(*byte_weights.get(&b'a').unwrap(), 5);
    /// assert_eq!(*byte_weights.get(&b'b').unwrap(), 5);
    /// assert_eq!(*byte_weights.get(&b'c').unwrap(), 1);
    /// ```
    #[derive(Clone, Copy, Eq)]
    pub struct ByteWeights{
        weights: [usize; 256],
        len: usize,
    }

    impl Weights<u8> for ByteWeights{
        fn get(&self, byte: &u8) -> Option<&usize>{
            self.get(byte)
        }

        fn get_mut(&mut self, byte: &u8) -> Option<&mut usize>{
            self.get_mut(byte)
        }

        fn len(&self) -> usize{
            self.len()
        }

        fn is_empty(&self) -> bool{
            self.is_empty()
        }
    }

    impl IntoIterator for ByteWeights{
        type Item = (u8, usize);
        type IntoIter = IntoIter;

        fn into_iter(self) -> IntoIter{
            IntoIter{weights: self, current_index: 0}
        }   
    }

    impl <'a> IntoIterator for &'a ByteWeights{
        type Item = (u8, usize);
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Iter<'a>{
            Iter{weights: &self, current_index: 0}
        }   
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

    impl ByteWeights{
        /// Initialize new `ByteWeights` from the given `&[u8]`
        /// 
        /// This algorithm is inherently O(n), therefore for
        /// larger collections `threaded_from_bytes` should be used.
        /// 
        /// # Example
        /// ---
        /// ```
        /// use huff_coding::prelude::ByteWeights;
        /// 
        /// let byte_weights = ByteWeights::from_bytes(b"aaaaa");
        /// assert_eq!(*byte_weights.get(&b'a').unwrap(), 5);
        /// ```
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

        /// Initialize new `ByteWeights` from the given `&[u8]`, but
        /// using the specified number of threads to speed up the
        /// process.
        /// 
        /// # Example
        /// ---
        /// ```
        /// use huff_coding::prelude::ByteWeights;
        /// 
        /// let byte_weights = ByteWeights::threaded_from_bytes(b"aaaaa", 12);
        /// assert_eq!(*byte_weights.get(&b'a').unwrap(), 5)
        /// ```
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

        /// Return a reference to the weight corresponding
        /// to the given byte.
        pub fn get(&self, byte: &u8) -> Option<&usize>{
            let weight = self.weights.get(*byte as usize)?;
            if *weight == 0{
                return None
            }
            Some(weight)
        }

        /// Return a mutable reference to the weight corresponding
        /// to the given byte.
        pub fn get_mut(&mut self, byte: &u8) -> Option<&mut usize>{
            let weight = self.weights.get_mut(*byte as usize)?;
            if *weight == 0{
                return None
            }
            Some(weight)
        }

        /// Return the number of different counted bytes stored in the `ByteWeights`
        pub fn len(&self) -> usize{
            self.len
        }

        /// Return true if len == 0
        pub fn is_empty(&self) -> bool{
            self.len == 0
        }

        /// Returns an iterator over the bytes to their weights `(u8, usize)`
        pub fn iter(&self) -> Iter{
            self.into_iter()
        }

        /// Add another `ByteWeights` to self, like so:
        /// * if a byte is present in self & other, add their weights
        /// * if a byte is present in other, but not in self, add it to self with other's weight
        /// 
        /// # Example
        /// –––
        /// ```
        /// use huff_coding::prelude::ByteWeights;
        /// 
        /// let mut byte_weights = ByteWeights::from_bytes(b"aabbb");
        /// let other = ByteWeights::from_bytes(b"aaabbc");
        /// 
        /// byte_weights.add_byte_weights(&other);
        /// 
        /// assert_eq!(*byte_weights.get(&b'a').unwrap(), 5);
        /// assert_eq!(*byte_weights.get(&b'b').unwrap(), 5);
        /// assert_eq!(*byte_weights.get(&b'c').unwrap(), 1);
        /// ```
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
}

use super::tree::HuffLetter;



pub trait Freq<L: HuffLetter>: PartialEq + Eq + Clone + IntoIterator<Item = (L, usize)>{
    fn get(&self, letter: &L) -> Option<&usize>;
    fn get_mut(&mut self, letter: &L) -> Option<&mut usize>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<L: HuffLetter> Freq<L> for std::collections::HashMap<L, usize>{
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

pub mod byte_freqs{
    use std::{
        ops::{Add, AddAssign},
        thread,
    };

    use crate::utils::ration_vec;
    use super::Freq;



    /// Struct used to count and store the 
    /// frequencies of bytes in a given byte slice.
    /// ---
    /// 
    /// Can be initialized either linearly:
    /// 
    /// ```
    /// use huff_encoding::prelude::ByteFreqs;
    /// let foo = ByteFreqs::from_bytes("bar".as_bytes());
    /// ```
    /// or threaded (faster for larger byte collections):
    /// 
    /// ```
    /// use huff_encoding::prelude::ByteFreqs;
    /// let foo = ByteFreqs::from_bytes("bar".as_bytes());
    /// ```
    #[derive(Clone, Copy, Eq)]
    pub struct ByteFreqs{
        freqs: [usize; 256],
        len: usize,
    }

    impl PartialEq for ByteFreqs{
        fn eq(&self, other: &Self) -> bool {
            self.freqs == other.freqs
        }
    }

    impl Add for ByteFreqs{
        type Output = Self;

        fn add(mut self, other: Self) -> Self {
            self.add_byte_freqs(&other);
            self
        }
    }

    impl AddAssign for ByteFreqs{
        fn add_assign(&mut self, other: Self){
            self.add_byte_freqs(&other);
        }
    }

    impl Freq<u8> for ByteFreqs{
        /// Return a reference to the frequency corresponding
        /// to the given byte.
        fn get(&self, byte: &u8) -> Option<&usize>{
            let freq = self.freqs.get(*byte as usize)?;
            if *freq == 0{
                return None
            }
            Some(freq)
        }

        /// Return a mutable reference to the frequency corresponding
        /// to the given byte.
        fn get_mut(&mut self, byte: &u8) -> Option<&mut usize>{
            let freq = self.freqs.get_mut(*byte as usize)?;
            if *freq == 0{
                return None
            }
            Some(freq)
        }

        /// Return the length of the wrapped Hashmap<u8; usize>.
        fn len(&self) -> usize{
            self.len
        }

        /// Return true if len == 0
        fn is_empty(&self) -> bool{
            self.len == 0
        }
    }


    /// Consuming iterator over the contents of ByteFreqs 
    /// 
    /// *(u8, usize)*
    pub struct Iter{
        freqs: ByteFreqs,

        current_index: usize,
    }

    impl Iterator for Iter{
        type Item = (u8, usize);

        fn next(&mut self) -> Option<Self::Item>{
            if self.current_index == 256{
                return None
            }

            while self.freqs.get(&(self.current_index as u8)).is_none(){
                if self.current_index == 256{
                    return None
                }
                self.current_index += 1
            }
            let entry = Some((self.current_index as u8, *self.freqs.get(&(self.current_index as u8)).unwrap()));
            if self.current_index != 256{self.current_index += 1;}

            entry
        }
    }

    impl IntoIterator for ByteFreqs{
        type Item = (u8, usize);
        type IntoIter = Iter;

        fn into_iter(self) -> Iter{
            Iter{freqs: self, current_index: 0}
        }   
    }

    /// Non consuming iterator over the contents of ByteFreqs 
    /// 
    /// *(u8, usize)*
    pub struct IterRef<'a>{
        freqs: &'a ByteFreqs,

        current_index: usize,
    }

    impl Iterator for IterRef<'_>{
        type Item = (u8, usize);

        fn next(&mut self) -> Option<Self::Item>{
            if self.current_index == 256{
                return None
            }

            while self.freqs.get(&(self.current_index as u8)).is_none(){
                if self.current_index == 256{
                    return None
                }
                self.current_index += 1
            }
            let entry = Some((self.current_index as u8, *self.freqs.get(&(self.current_index as u8)).unwrap()));
            if self.current_index != 256{self.current_index += 1;}

            entry
        }
    }

    impl <'a> IntoIterator for &'a ByteFreqs{
        type Item = (u8, usize);
        type IntoIter = IterRef<'a>;

        fn into_iter(self) -> IterRef<'a>{
            IterRef{freqs: &self, current_index: 0}
        }   
    }


    impl ByteFreqs{
        /// Count all bytes in given slice and organize them
        /// into ```ByteFreqs``` (internally a lookup table)
        /// 
        /// Threaded version is faster for bigger files (huff_encoding::ByteFreqs::threaded_from_bytes).
        /// 
        /// # Examples
        /// ---
        /// ```
        /// use huff_encoding::prelude::ByteFreqs;
        /// 
        /// let foo = ByteFreqs::from_bytes("bar".as_bytes());
        /// ```
        pub fn from_bytes(bytes: &[u8]) -> Self{
            // count bytes into an array
            let mut byte_freqs: [usize; 256] = [0;256];
            let mut len = 0;

            for byte in bytes{
                if byte_freqs[*byte as usize] == 0{len += 1;}
                byte_freqs[*byte as usize] += 1;
            }

            // convert the array into a hashmap
            ByteFreqs{
                freqs: byte_freqs,
                len,
            }
        }

        /// Count all bytes in given slice and organize them
        /// into ByteFreqs (internally a lookup table)
        /// Uses multiple threads (it's faster for bigger files).
        ///   
        /// Non-threaded version is faster for smaller files (huff_encoding::ByteFreqs::from_bytes).
        /// 
        /// # Examples
        /// ---
        /// ```
        /// use huff_encoding::prelude::ByteFreqs;
        /// 
        /// let foo = ByteFreqs::threaded_from_bytes("bar".as_bytes(), 12 /* number of threads */);
        /// ```
        pub fn threaded_from_bytes(bytes: &[u8], thread_num: usize) -> Self{
            // divide the bytes into rations per thread
            let byte_rations = ration_vec(bytes, thread_num);

            // create ByteFreqs from every ration
            let mut handles = Vec::with_capacity(thread_num);
            for ration in byte_rations{
                let handle = thread::spawn(move || {
                    ByteFreqs::from_bytes(&ration)
                });
                handles.push(handle);
            }

            // push all created ByteFreqs into a Vec 
            let mut byte_freq_vec: Vec<ByteFreqs> = Vec::with_capacity(thread_num);
            for handle in handles{
                byte_freq_vec.push(handle.join().unwrap());
            }

            // add all ByteFreqs into one
            let mut byte_freqs = byte_freq_vec.pop().unwrap();
            for byte_freqs_other in byte_freq_vec{
                byte_freqs += byte_freqs_other;
            }

            byte_freqs
        }

        /// Returns an iterator over the bytes to their frequencies
        /// 
        /// *(u8, usize)*
        pub fn iter(&self) -> IterRef{
            self.into_iter()
        }

        /// Add another ByteFreqs to self
        pub fn add_byte_freqs(&mut self, other: &ByteFreqs){
            for (b, f) in other{
                let self_entry = self.get_mut(&b);
                match self_entry{
                    Some(self_entry) =>{
                        *self_entry += f;
                    }
                    None =>{
                        self.freqs[b as usize] = f;
                        self.len += 1;
                    }
                }
            }
        }
    }
}

use std::thread;

use crate::utils::ration_vec;



/// Struct used to count and store the 
/// frequencies of bytes in a given &[u8]
/// ---
/// 
/// Can be initialized either linearly:
/// 
/// ```
/// use huff_encoding::ByteFreqs;
/// let foo = ByteFreqs::from_bytes(&"bar".as_bytes());
/// ```
/// or threaded (faster for larger byte collections):
/// 
/// ```
/// use huff_encoding::ByteFreqs;
/// let foo = ByteFreqs::from_bytes(&"bar".as_bytes());
/// ```
pub struct ByteFreqs{
    freqs: [usize; 256],
    len: usize,
}

/// Iterator over the contents of ByteFreqs (byte, freq)2
pub struct ByteFreqsIter<'a>{
    freqs: &'a ByteFreqs,

    current_index: usize,
}

impl Iterator for ByteFreqsIter<'_>{
    type Item = (u8, usize);

    fn next(&mut self) -> Option<Self::Item>{
        if self.current_index == 256{
            return None
        }

        while self.freqs.get(self.current_index as usize).is_none(){
            if self.current_index == 256{
                return None
            }
            self.current_index += 1
        }
        let entry = Some((self.current_index as u8, *self.freqs.get(self.current_index as usize).unwrap()));
        if self.current_index != 256{self.current_index += 1;}

        entry
    }
}

impl <'a> IntoIterator for &'a ByteFreqs{
    type Item = (u8, usize);
    type IntoIter = ByteFreqsIter<'a>;

    fn into_iter(self) -> ByteFreqsIter<'a>{
        ByteFreqsIter{freqs: &self, current_index: 0}
    }   
}

impl ByteFreqs{
    /// Count all bytes in given slice and organize them
    /// into ByteFreqs (internally a lookup table)
    /// 
    /// Threaded version is faster for bigger files (huff_encoding::ByteFreqs::threaded_from_bytes).
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::ByteFreqs;
    /// 
    /// let foo = ByteFreqs::from_bytes("bar".as_bytes());
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> ByteFreqs{
        // count bytes into an array
        let mut byte_freqs: [usize; 256] = [0;256];
        let mut len = 0;

        let mut previous_byte: Option<u8> = None;
        for b in bytes{
            if previous_byte != Some(*b){len += 1;}
            byte_freqs[*b as usize] += 1;
            previous_byte = Some(*b)
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
    /// use huff_encoding::ByteFreqs;
    /// 
    /// let foo = ByteFreqs::from_bytes("bar".as_bytes());
    /// ```
    pub fn threaded_from_bytes(bytes: &[u8]) -> ByteFreqs{
        // divide the bytes into rations per thread
        let byte_rations = ration_vec(&bytes.to_vec(), num_cpus::get());

        // create ByteFreqs from every ration
        let mut handles = Vec::with_capacity(num_cpus::get());
        for ration in byte_rations{
            let handle = thread::spawn(move || {
                ByteFreqs::from_bytes(&ration)
            });
            handles.push(handle);
        }

        // push all created ByteFreqs into a Vec 
        let mut bfreq_mult: Vec<ByteFreqs> = Vec::with_capacity(num_cpus::get());
        for handle in handles{
            bfreq_mult.push(handle.join().unwrap());
        }

        // add all ByteFreqs into one
        let mut byte_freqs = bfreq_mult.pop().unwrap();
        for bfreq in bfreq_mult{
            byte_freqs.add_bfreq(&bfreq);
        }

        ByteFreqs{
            freqs: byte_freqs.freqs,
            len: byte_freqs.len
        }
    }


    /// Return a reference to the frequency corresponding
    /// to the given byte.
    pub fn get(&self, b: usize) -> Option<&usize>{
        let entry = self.freqs.get(b);
        match entry{
            Some(_) =>{
                if *entry.unwrap() == 0{
                    return None
                }
                entry
            }
            None => None,
        }
    }

    /// Return a mutable reference to the frequency corresponding
    /// to the given byte.
    pub fn get_mut(&mut self, b: usize) -> Option<&mut usize>{
        let entry = self.freqs.get_mut(b);
        match entry{
            Some(_) =>{
                let freq = entry.unwrap(); 
                if *freq == 0{
                    return None
                }
                Some(freq)
            }
            None => None
        }
    }

    /// Return the length of the wrapped Hashmap<u8; usize>.
    pub fn len(&self) -> usize{
        self.len
    }

    pub fn is_empty(&self) -> bool{
        self.len == 0
    }

    /// Add another ByteFreqs to self
    pub fn add_bfreq(&mut self, other: &ByteFreqs){
        for (b, f) in other{
            let self_entry = self.get_mut(b as usize);
            match self_entry{
                Some(_) =>{
                    *self_entry.unwrap() += f;
                }
                None =>{
                    self.freqs[b as usize] = f;
                    self.len += 1;
                }
            }
        }
    }
}

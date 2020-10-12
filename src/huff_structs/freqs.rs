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
/// let foo = ByteFreqs::from("bar".as_bytes());
/// ```
/// or threaded (faster for larger byte collections):
/// 
/// ```
/// use huff_encoding::ByteFreqs;
/// let foo = ByteFreqs::from("bar".as_bytes());
/// ```
pub struct ByteFreqs{
    freqs: [usize; 256],
} 

impl ByteFreqs{
    /// Count all bytes in given slice and organize them
    /// into ByteFreqs (internally a lookup table)
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::ByteFreqs;
    /// 
    /// let foo = ByteFreqs::from("bar".as_bytes());
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> ByteFreqs{
        // count bytes into an array
        let mut byte_freqs: [usize; 256] = [0;256];
        for b in bytes{
            byte_freqs[*b as usize] += 1;
        }

        // convert the array into a hashmap
        return ByteFreqs{
            freqs: byte_freqs
        };
    }

    /// Count all bytes in given slice and organize them
    /// into ByteFreqs (internally a lookup table)
    ///   
    /// ### Uses multiple threads to count bytes faster 
    /// ---
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::ByteFreqs;
    /// 
    /// let foo = ByteFreqs::from("bar".as_bytes());
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

        return ByteFreqs{
            freqs: byte_freqs.freqs,
        };
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
                return entry
            }
            None => return None
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
                return Some(freq)
            }
            None => return None
        }
    }

    /// Return the length of the wrapped Hashmap<u8; usize>.
    pub fn len(&self) -> usize{
        return self.freqs.len();
    }


    /// Return an Iterator over the contents of ByteFreqs (yields a tuple (byte, freq))
    pub fn iter(&self) -> std::iter::Enumerate<std::slice::Iter<'_, usize>>{
        return self.freqs.iter().enumerate();
    }

    /// Return a mutable Iterator over the contents of ByteFreqs (yields a tuple (byte, freq))
    pub fn iter_mut(&mut self) -> std::iter::Enumerate<std::slice::IterMut<'_, usize>>{
        return self.freqs.iter_mut().enumerate();
    }


    /// Add another ByteFreqs to self
    pub fn add_bfreq(&mut self, other: &ByteFreqs){
        for (b, f) in other.iter(){
            let self_entry = self.get_mut(b);
            match self_entry{
                Some(_) =>{
                    *self_entry.unwrap() += f;
                }
                None =>{
                    self.freqs[b] = *f;
                }
            }
        }
    }
}

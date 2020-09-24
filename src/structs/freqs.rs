use std::collections::HashMap;
use std::thread;

use crate::utils::ration_vec;



#[derive(Debug)]
pub struct ByteFreqs{
    freqs: HashMap<u8, usize>,
} 

impl ByteFreqs{
    /// Count all bytes in given slice and organize them
    /// into ByteFreqs (internally a Hashmap<u8, usize>)
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::ByteFreqs;
    /// 
    /// let foo = ByteFreqs::from("bar".as_bytes());
    /// ```
    pub fn from(bytes: &[u8]) -> ByteFreqs{
        // count bytes into an array
        let mut byte_freqs: [usize; 256] = [0;256];
        for b in bytes{
            byte_freqs[*b as usize] += 1;
        }

        // convert the array into a hashmap
        return ByteFreqs{
            freqs: {
                let mut h: HashMap<u8, usize> = HashMap::new();
                for (b, freq) in byte_freqs.iter().enumerate(){
                    if *freq != 0{
                        h.insert(b as u8, *freq);
                    }
                }
                h
            }
        };
    }

    /// Count all bytes in given slice and organize them
    /// into ByteFreqs (internally a Hashmap<u8, usize>)
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
    pub fn threaded_from(bytes: &[u8]) -> ByteFreqs{
        // divide the bytes into rations per thread
        let byte_rations = ration_vec(bytes.to_vec(), num_cpus::get());
        
        // create ByteFreqs from every ration
        let mut handles = vec![];
        for ration in byte_rations{
            let handle = thread::spawn(move || {
                ByteFreqs::from(&ration)
            });
            handles.push(handle);
        }

        // push all created ByteFreqs into a Vec 
        let mut bfreq_mult: Vec<ByteFreqs> = Vec::new();
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
    pub fn get(&self, b: &u8) -> Option<&usize>{
        return self.freqs.get(b);
    }

    /// Return a mutable reference to the frequency corresponding
    /// to the given byte.
    pub fn get_mut(&mut self, b:&u8) -> Option<&mut usize>{
        return self.freqs.get_mut(b);
    }

    /// Return the length of the wrapped Hashmap<u8; usize>.
    pub fn len(&self) -> usize{
        return self.freqs.len();
    }


    /// Return an Iterator over the contents of ByteFreqs
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u8, usize>{
        return self.freqs.iter();
    }

    /// Return a mutable Iterator over the contents of ByteFreqs
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, u8, usize>{
        return self.freqs.iter_mut();
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
                    self.freqs.insert(*b, *f);
                }
            }
        }
    }
}
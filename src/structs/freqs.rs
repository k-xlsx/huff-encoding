use std::collections::HashMap;
use std::thread;



#[derive(Debug)]
pub struct ByteFreqs{
    freqs: HashMap<u8, usize>,
} 

impl ByteFreqs{
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

    pub fn threaded_from(bytes: &[u8]) -> ByteFreqs{
        // divide the bytes into rations per thread
        let thread_count = num_cpus::get();
    
        let mut bytes_left = bytes.len();
        let bytes_per_thread = bytes_left / thread_count;
        let mut current_byte = 0;
    
        let mut byte_rations: Vec<Vec<u8>> = Vec::new();
        if bytes_per_thread == 0{
            byte_rations.push(bytes[..].to_vec());
        }
        else{
            for _ in 0..thread_count{
                if bytes_left < bytes_per_thread{
                    byte_rations.push(bytes[current_byte..].to_vec());
                    break;
                }
        
                byte_rations.push(bytes[current_byte..current_byte + bytes_per_thread].to_vec());
                current_byte += bytes_per_thread;
                bytes_left -= bytes_per_thread;
            }
        }
        
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


    pub fn get(&self, b: &u8) -> Option<&usize>{
        return self.freqs.get(b);
    }

    pub fn get_mut(&mut self, b:&u8) -> Option<&mut usize>{
        return self.freqs.get_mut(b);
    }

    pub fn len(&self) -> usize{
        return self.freqs.len();
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u8, usize>{
        return self.freqs.iter();
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, u8, usize>{
        return self.freqs.iter_mut();
    }

  
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
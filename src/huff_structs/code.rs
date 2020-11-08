use std::hash::{Hash, Hasher};

/// Struct used to store the code of
/// a given HuffBranch. HuffCode is 
/// accessed bitwise.
/// ---
/// Internal storage:
/// 
/// [u64; 4]
/// ---
/// Max length of a HuffCode in my
/// implementation is 255, as I'm using
/// bytes as my alphabet and max length equals:
///  
/// *alphabet_size - 1*
#[derive(Debug, Clone, Copy, Eq)]
pub struct HuffCode{
    storage: [u64; 4],

    next_bit: u8,
    current_block: u8,
    len: usize,
}

impl Hash for HuffCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..=self.current_block as usize{
            self.storage[i].hash(state);
        }
    }
}

impl Default for HuffCode{
    fn default() -> Self{
        Self::new()
    }
}

impl PartialEq for HuffCode{
    fn eq(&self, other: &Self) -> bool {
        self.storage == other.storage && self.len == other.len
    }
}

/// Iterator over the contents of a HuffCode.
pub struct HuffCodeIter<'a>{
    code: &'a HuffCode,

    current_index: u8,
}

impl Iterator for HuffCodeIter<'_>{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item>{
        let value = self.code.get(self.current_index as usize);
        self.current_index += 1;
        value
    }
}

impl<'a> IntoIterator for &'a HuffCode{
    type Item = bool;
    type IntoIter = HuffCodeIter<'a>;

    fn into_iter(self) -> HuffCodeIter<'a>{
        HuffCodeIter{code: &self, current_index: 0}
    }
}

impl HuffCode{
    /// Initializes an empty HuffCode.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::HuffCode;
    /// 
    /// let foo = HuffCode::new();
    /// ```
    pub fn new() -> HuffCode{
        HuffCode{
            storage: [0; 4],

            next_bit: 0,
            current_block: 0,
            len: 0,
        }
    }

    /// Returns the bit stored at the given index.
    ///   
    /// If index is out of bounds, returns None.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::HuffCode;
    /// 
    /// let mut code = HuffCode::new();
    /// code.push(true);
    /// 
    /// // does not panic.
    /// assert_eq!(code.get(0).unwrap(), true);
    /// assert_eq!(code.get(1), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<bool>{
        if index >= self.len{
            return None;
        }

        let block_index = HuffCode::get_block_index(index);
        let index_in_block = HuffCode::get_rel_index(index, block_index);

        let block = self.storage[block_index];
        Some((block >> (63 - index_in_block)) % 2 != 0)
    }

    /// Pushes a bit at the end of the code.
    pub fn push(&mut self, bit: bool){
        assert!(!(self.current_block == 3 && self.next_bit == 64), "tried to push over max capacity");
        
        self.set_next_bit(bit);

        // increment length and all pointers
        self.len += 1;
        self.next_bit += 1;
        if self.next_bit == 64 && self.current_block != 3{
            self.next_bit = 0;
            self.current_block += 1;
        }
    }

    /// Removes the last bit and returns it, or None if it is empty.
    pub fn pop(&mut self) -> Option<bool>{
        match !self.is_empty(){
            true =>{
                // decrement the lenght and all pointers
                self.len -= 1;
                if self.next_bit == 0{
                    self.current_block -= 1;
                    self.next_bit = 63;
                }
                else{
                    self.next_bit -= 1;
                }

                // read the value of the bit to be popped
                let popped_bit = Some((self.storage[self.current_block as usize] >> (63 - self.next_bit)) % 2 != 0);

                // set the bit to 0
                self.set_next_bit(false);

                popped_bit
            }
            false => None,
        }

    }

    /// Clears the code of all bits
    pub fn clear(&mut self){
        for i in 0..=self.current_block as usize{
            self.storage[i] = 0;
        }

        self.len = 0;
        self.next_bit = 0;
        self.current_block = 0;
    }

    /// Returns the length of the code (in bits of course).
    pub fn len(&self) -> usize{
        self.len
    }

    /// Return true if len == 0
    pub fn is_empty(&self) -> bool{
        self.len == 0 
    }

    /// Sets self.storage[self.current_block] at bit self.next_bit to bit 
    fn set_next_bit(&mut self, bit: bool){
        if bit{
            self.storage[self.current_block as usize] |= 1 << (63 - self.next_bit);
        }
        else{
            self.storage[self.current_block as usize] &= !(1 << (63 - self.next_bit));
        }
    }

    /// Returns the index of the block that the given bit's in
    fn get_block_index(index: usize) -> usize{
        let i = index + 1; 
        (i / 64 + (i % 64 != 0) as usize) - 1
    }

    /// Returns the index relative to the given block.
    fn get_rel_index(abs_index: usize, block_index: usize) -> usize{
        abs_index - block_index * 64
    }
}


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
/// 
#[derive(Debug, Clone, Eq, Hash)]
pub struct HuffCode{
    storage: [u64; 4],

    next_bit: u8,
    current_block: u8,
    len: usize,
}

impl PartialEq for HuffCode{
    fn eq(&self, other: &Self) -> bool {
        return self.storage == other.storage;
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
        return value;
    }
}

impl<'a> IntoIterator for &'a HuffCode{
    type Item = bool;
    type IntoIter = HuffCodeIter<'a>;

    fn into_iter(self) -> HuffCodeIter<'a>{
        return HuffCodeIter{code: &self, current_index: 0}
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
        return HuffCode{
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
        return Some((block >> (63 - index_in_block)) % 2 != 0);
    }

    /// Pushes a bit at the end of the code.
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
    /// ```
    pub fn push(&mut self, bit: bool){
        assert!(!(self.current_block == 3 && self.next_bit == 64), "tried to push over max capacity");

        if bit{
            let mut block = self.storage[self.current_block as usize];
            block |= 1 << (63 - self.next_bit);
            self.storage[self.current_block as usize] = block;
        }
        self.len += 1;
        self.next_bit += 1;
        if self.next_bit == 64{
            if self.current_block != 3{
                self.next_bit = 0;
                self.current_block += 1;
            }
        }
    }


    /// Returns the length of the code (in bits of course).
    pub fn len(&self) -> usize{
        return self.len;
    }

    /// Returns the index of the block that the given bit's in
    fn get_block_index(index: usize) -> usize{
        let i = index + 1; 
        return (i / 64 + (i % 64 != 0) as usize) - 1;
    }

    /// Returns the index relative to the given block.
    fn get_rel_index(abs_index: usize, block_index: usize) -> usize{
        return abs_index - block_index * 64;
    }
}

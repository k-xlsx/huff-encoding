use bitvec::prelude::{BitVec, LocalBits};



/// Struct used to store HuffBranch data:
/// ```
/// byte: Option<u8>;                // get
/// frequency: usize                 // get
/// code: Option<bit_vec::BitVec>;   // get&set
/// ```
#[derive(Debug, Clone, Eq)]
pub struct HuffLeaf{
    byte: Option<u8>,
    frequency: usize,
    code: Option<BitVec::<LocalBits, usize>>,
}

impl PartialEq for HuffLeaf {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl HuffLeaf{
    /// Initialize the HuffLeaf.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::HuffLeaf;
    /// 
    /// let foo = HuffLeaf::new(0xc4, 3);
    /// ```
    pub fn new(byte: Option<u8>, frequency: usize) -> HuffLeaf{
        let huff_leaf = HuffLeaf{
            byte: byte,
            frequency: frequency,
            code: None,
        };

        return huff_leaf;
    }


    /// Returns the stored byte.
    pub fn byte(&self) -> Option<u8>{
        return self.byte;
    }
    
    /// Returns the stored frequency.
    pub fn frequency(&self) -> usize{
        return self.frequency
    }
    
    /// Returns a reference to the stored code.
    pub fn code(&self) -> Option<&BitVec>{
        return self.code.as_ref();
    }


    /// Sets the given BitVec as code.
    pub fn set_code(&mut self, code: BitVec){    
        self.code = Some(code);
    }
}

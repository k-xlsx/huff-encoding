use bit_vec::BitVec;



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
    code: Option<BitVec>,
}

impl PartialEq for HuffLeaf {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl HuffLeaf{
    pub fn new(byte: Option<u8>, frequency: usize) -> HuffLeaf{
        //! Initialize the HuffLeaf.
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::HuffLeaf;
        //! 
        //! let foo = HuffLeaf::new(0xc4, 3);
        //! ```


        let huff_leaf = HuffLeaf{
            byte: byte,
            frequency: frequency,
            code: None,
        };

        return huff_leaf;
    }

    
    pub fn byte(&self) -> Option<u8>{
        //! Returns the stored byte.


        return self.byte;
    }
    
    pub fn frequency(&self) -> usize{
        //! Returns the stored frequency.


        return self.frequency
    }
    
    pub fn code(&self) -> Option<&BitVec>{
        //! Returns a reference to the stored code.
        

        return self.code.as_ref();
    }


    pub fn set_code(&mut self, code: BitVec){
        //! Sets the given BitVec as code.
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::HuffLeaf;
        //! 
        //! let b = bit_vec::BitVec::new();
        //! b.push(true)
        //! 
        //! foo.set_code(b);
        //! ```

        
        self.code = Some(code);
    }
}

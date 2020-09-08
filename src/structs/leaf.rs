use bit_vec::BitVec;



/// Struct used to store HuffBranch data:
/// ```
/// character: Option<char>;         // get
/// frequency: usize                 // get
/// code: Option<bit_vec::BitVec>;   // get&set
/// ```
#[derive(Debug, Clone, Eq)]
pub struct HuffLeaf{
    character: Option<char>,
    frequency: usize,
    code: Option<BitVec>,
}

impl PartialEq for HuffLeaf {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl HuffLeaf{
    pub fn new(character: Option<char>, frequency: usize) -> HuffLeaf{
        //! Initialize the HuffLeaf.
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffLeaf;
        //! 
        //! let foo = HuffLeaf::new('s', 3);
        //! ```


        let huff_leaf = HuffLeaf{
            character: character,
            frequency: frequency,
            code: None,
        };

        return huff_leaf;
    }

    
    pub fn character(&self) -> Option<char>{
        //! Returns the stored character.


        return self.character;
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
        //! use huff_encoding::huff_structs::HuffLeaf;
        //! 
        //! let b = bit_vec::BitVec::new();
        //! b.push(true) 
        //! 
        //! foo.set_code(b);
        //! ```

        
        self.code = Some(code);
    }
}

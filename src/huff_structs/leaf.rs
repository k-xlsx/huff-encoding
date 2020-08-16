#![allow(dead_code)]



/// Struct used to store HuffBranch data:
/// ```
/// character: Option<char>;         // get
/// frequency: usize                 // get
/// code: Option<String>;            // get/set
/// ```
#[derive(Debug, Clone, Eq)]
pub struct HuffLeaf{
    character: Option<char>,
    frequency: usize,
    code: Option<String>,
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
    
    pub fn code(&self) -> Option<&String>{
        //! Returns a reference to the stored code.
        

        return self.code.as_ref();
    }


    pub fn set_code(&mut self, code: &str){
        //! Sets the given code.
        //! 
        //! Panics if code is not binary.
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffLeaf;
        //! 
        //! foo.set_code("101001");
        //! ```

        
        HuffLeaf::check_code(&code);
        self.code = Some(code.to_string());
    }


    fn check_code(code: &str){
        for c in code.chars(){
            if c != '1' && c != '0'{
                panic!("given code String is not binary");
            }
        }
    }
}

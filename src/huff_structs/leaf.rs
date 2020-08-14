#![allow(dead_code)]


/// Struct used to store HuffBranch data:
/// ```
/// character: Option<char>;         // get
/// frequency: u32                   // get
/// code: Option<String>;            // get/set
/// ```
#[derive(Debug, Clone, Eq)]
pub struct HuffLeaf{
    character: Option<char>,
    frequency: u32,
    code: Option<String>,
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
    /// use huff_encoding::huff_structs::HuffLeaf;
    /// 
    /// let hf = HuffLeaf::new('s', 3);
    /// ```
    pub fn new(character: Option<char>, frequency: u32) -> HuffLeaf{
        let huff_leaf = HuffLeaf{
            character: character,
            frequency: frequency,
            code: None,
        };

        return huff_leaf;
    }

    /// Returns the stored character.
    pub fn character(&self) -> Option<char>{
        return self.character;
    }
    
    /// Returns the stored frequency.
    pub fn frequency(&self) -> u32{
        return self.frequency
    }
    
    /// Returns a reference to the stored code.
    pub fn code(&self) -> Option<&String>{
        return self.code.as_ref();
    }

    /// Sets the given code.
    /// 
    /// Panics if code is not binary.
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::huff_structs::HuffLeaf;
    /// 
    /// huff_leaf.set_code("101001");
    /// ```
    pub fn set_code(&mut self, code: &str){
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

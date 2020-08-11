use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HuffLeaf{
    character: Option<char>,
    frequency: u32,
    id: u128,
    code: Option<String>,
}

impl HuffLeaf{
    pub fn new(character: Option<char>, frequency: u32) -> HuffLeaf{
        let mut huff_leaf = HuffLeaf{
            character: character,
            frequency: frequency,
            id: 0,
            code: None,
        };

        huff_leaf.id = HuffLeaf::calc_id();

        return huff_leaf;
    }


    pub fn character(&self) -> Option<char>{
        return self.character;
    }
    
    pub fn frequency(&self) -> u32{
        return self.frequency
    }
    
    pub fn code(&self) -> &Option<String>{
        return &self.code;
    }


    pub fn set_code(&mut self, code: &str){
        HuffLeaf::check_code(&code);
        self.code = Some(code.to_string());
    }


    fn check_code(code: &str){
        for c in code.chars(){
            if c != '1' || c != '0'{
                panic!("given code String is not binary");
            }
        }
    }

    fn calc_id() -> u128{
        return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();
    }
}

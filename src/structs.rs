mod leaf;
mod branch;
mod branch_heap;
mod tree;


pub use self::leaf::HuffLeaf;
pub use self::branch::HuffBranch;
pub use self::tree::HuffTree;

use std::collections::HashMap;



pub fn chars_to_freq(s: &str) -> HashMap<char, usize>{
    //! Returns a HashMap of chars to their corresponding
    //! frequencies in the given &str
    //! 
    //! # Examples
    //! ```
    //! use huff_structs::chars_to_freq;
    //! 
    //! let foo = chars_to_freq("Hello World");
    //! print!("{:#?}", foo);
    //! 
    //! /// outputs something like:
    //! /// {
    //! ///     ' ': 1,
    //! ///     'l': 3,
    //! ///     'o': 2,
    //! ///     'H': 1,
    //! ///     'd': 1,
    //! ///     'e': 1,
    //! ///     'W': 1,
    //! ///     'r': 1
    //! /// }
    //! ```


    let mut ctf: HashMap<char, usize> = HashMap::new();

    for c in s.chars(){
        let cf_entry = ctf.entry(c).or_insert(0);
        *cf_entry += 1;
    }

    return ctf;
}

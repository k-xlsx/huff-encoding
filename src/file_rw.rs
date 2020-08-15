use crate::huff_structs::HuffTree;
// use bitbit::{BitReader, BitWriter};



pub fn hfc_prefix(h_tree: &HuffTree) -> String{
    let char_codes = h_tree.char_codes();

    
    let mut hfc_prefix = String::new();
    for (c, code) in char_codes{
        let c_bin = format!("{:032b}", *c as u32);
        hfc_prefix.push_str(&c_bin[..]);
        hfc_prefix.push_str(code);
    }

    let prefix_len = format!("{:064b}", hfc_prefix.len());

    return format!("{}{}", prefix_len, hfc_prefix);
}

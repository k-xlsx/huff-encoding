// // TODO: finish this

// use super::{
//     HuffTree,
// }; 



// pub fn compress(bytes: &[u8]){
//     let huff_tree = HuffTree::from_bytes(bytes);

//     println!("{:?}", huff_tree.byte_codes());
//     let compressed_bytes = get_compressed_bytes(bytes, &huff_tree).unwrap();
//     println!("{:?},{:?}", compressed_bytes.0, compressed_bytes.1);
// }

// pub fn get_compressed_bytes(bytes: &[u8], huff_tree: &HuffTree) -> Option<(Vec<u8>, u8)>{
//     let byte_codes = huff_tree.byte_codes();

//     let mut codes: Vec<HuffCode> = Vec::with_capacity(bytes.len());
//     for byte in bytes{
//         let byte_codes_entry = byte_codes.get(byte);
//         if let Some(byte_code) = byte_codes_entry{
//             codes.push(*byte_code)
//         }
//         else{
//             return None
//         }
//     }
//     Some(huff_codes_to_bytes(&codes))
// }

// fn huff_codes_to_bytes(coded_bytes: &[HuffCode]) -> (Vec<u8>, u8){
//     let mut bytes = Vec::with_capacity(coded_bytes.len() / 2);
//     let mut current_byte = 0b0000_0000;
//     let mut i = 7;
//     for code in coded_bytes{
//         for bit in code.iter(){
//             current_byte |= (bit as u8) << i;
//             if i == 0{
//                 bytes.push(current_byte);
//                 current_byte = 0b0000_0000;
//                 i = 7;
//             }
//             else{i -= 1};
//         }
//     }
//     let padding_bits = if i == 7{0} else{i + 1};
//     if padding_bits != 0{bytes.push(current_byte);}
//     (bytes, padding_bits)
// }


// pub fn decompress(bytes: &[u8]){
// }
/// Move the provided bytes to the right by n bits
pub fn offset_bytes(bytes: &[u8], n: usize) -> Vec<u8>{
    let empty_bytes = n / 8;
    let mut offset_bytes = vec![0; empty_bytes];
    offset_bytes.reserve_exact(bytes.len());

    let mut comp_byte = 0b0000_0000;
    let mut bit_ptr = (7 - n) % 8;
    for byte in bytes{
        for i in 0..8{
            comp_byte |= (((byte >> (7 - i)) & 1 == 1) as u8) << bit_ptr;

            if bit_ptr == 0{
                offset_bytes.push(comp_byte);
                comp_byte = 0b0000_0000;
                bit_ptr = 7;
            }
            else{bit_ptr -= 1};
        }
    }
    let padding_bits = if bit_ptr == 7{0} else{bit_ptr + 1};
    if padding_bits != 0{offset_bytes.push(comp_byte);}

    offset_bytes
}

/// Return how many bits will be used as padding
/// with given the bit_count.
pub fn calc_padding_bits(bit_count: usize) -> u8{
    let n = (8 - bit_count % 8) as u8; 
    match n{8 => 0, _ => n}
}

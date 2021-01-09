use huff_coding::{
    prelude::*,
    bitvec::prelude::*,
};



#[test]
fn compress_decompress(){
    let bytes = b"float Q_rsqrt( float number )
    {
        long i;
        float x2, y;
        const float threehalfs = 1.5F;
    
        x2 = number * 0.5F;
        y  = number;
        i  = * ( long * ) &y;                       // evil floating point bit level hacking
        i  = 0x5f3759df - ( i >> 1 );               // what the fuck? 
        y  = * ( float * ) &i;
        y  = y * ( threehalfs - ( x2 * y * y ) );   // 1st iteration
    //	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed
    
        return y;
    }";

    let compressed = compress(bytes);
    let decompressed = decompress(&compressed).unwrap();

    assert_eq!(bytes.to_vec(), decompressed);
}

#[test]
fn get_compressed_decompressed(){
    let bytes = b"Java is a class-based, object-oriented programming language 
    that is designed to have as few implementation dependencies as possible. 
    It is a general-purpose programming language intended to let application developers 
    write once, run anywhere (WORA), meaning that compiled Java code can run on all platforms 
    that support Java without the need for recompilation. Java applications are typically compiled 
    to bytecode that can run on any Java virtual machine (JVM) regardless of the underlying 
    computer architecture. The syntax of Java is similar to C and C++, but has fewer low-level 
    facilities than either of them. The Java runtime provides dynamic capabilities 
    (such as reflection and runtime code modification) that are typically not available 
    in traditional compiled languages. As of 2019, Java was one of the most popular 
    programming languages in use according to GitHub, particularly for 
    client-server web applications, with a reported 9 million developers.";

    let tree = HuffTree::from_weights(ByteWeights::from_bytes(bytes));

    let (compressed, padding_bits) = get_compressed_bytes(bytes, &tree).unwrap();
    let decompressed = get_decompressed_bytes(&compressed, padding_bits, &tree);

    assert_eq!(bytes.to_vec(), decompressed);
}
# **huff**

[![Build Status](https://travis-ci.com/kxlsx/huffman-coding-rs.svg?branch=master)](https://travis-ci.com/k-xlsx/huffman-coding-rs)

Example compression/decompression CLI software based on the [**huff_coding**][lib] crate.

## Usage

```txt
huff [FLAGS] [OPTIONS] <SRC_FILE> [DST_FILE]
```

## Args

```txt
<SRC_FILE>    
<DST_FILE>    [default: ./SRC_FILE.hff]
```

## Options

```txt
-b, --block-size <SIZE>
        Set how many bytes can be loaded from the file at one time
        Possible units: 
            K/Ki -> Kilobytes/Kibibytes
            M/Mi -> Megabytes/Mebibytes
            G/Gi -> Gigabytes/Gibibytes
         [default: 2G]
```

## Flags

```txt
-d, --decompress    
        Decompresses the hff SRC_FILE into DST_FILE.hff
            
-n, --noask         
        Omits asking if should replace existing DST_FILE

-r, --replace       
        Deletes SRC_FILE upon completion

-t, --time          
        Prints how long it took to finish
-h, --help          
        Prints help information

-V, --version       
        Prints version information
```

## File format

The *hff* file format is encoded as follows:

1. A byte containing the number of bits used for padding:
   * first 4 bits store the [`HuffTree`'s][tree] padding bits
   * the remaining bits store the compressed data's padding bits
2. 4 byte number representing the length (in bytes) of the stored [`HuffTree`][tree]
3. A [`HuffTree`][tree], used to compress the file,
represented in binary (see [`HuffTree::try_from_bin`][tree_from_bin])
4. The actual compressed data

[lib]:https://github.com/kxlsx/huffman-coding-rs/tree/master/**huff_coding**
[tree]:https://github.com/kxlsx/huffman-coding-rs/blob/master/huff_coding/src/tree/mod.rs#L27
[tree_from_bin]:https://github.com/kxlsx/huffman-coding-rs/blob/master/huff_coding/src/tree/mod.rs#L452

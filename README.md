# huff-encoding

An implementation of the *[Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding)* algorithm in Rust.

Contains:

- An [executable](https://github.com/k-xlsx/huff-encoding/releases) with a basic CLI compression program,  
- A [library](https://github.com/k-xlsx/huff-encoding/tree/dev/src) containing methods to compress/decompress byte data, write/read compressed files, as well as structs representing a *Huffman Tree*

## bin

It's just basic compression/decompression software using *multithreading* by default.

**[WARNING]** Reading can be currently a bit slow, Windows version hasn't    been tested.

 Usage:

>     huff [FLAGS] <COMMAND>

Commands:

> - **compress** *[FLAGS] <src_path> [dst_path]*
>   - dst_path is *./src_filename.hfe* by default
>   - Flags
>     - -s, --single_thread -> Use only one thread to compress (Can be faster for smaller files)
> - **decompress** *<src_path> [dst_path]*
>   - dst_path is *./src_filename.extension_stored_in_compressed_file* by default
> - **help**

Flags:

- -h, --help -> Prints help information
- -t, --time  -> Show the time it took for a command to finish
- -V, --version ->    Prints version information

## lib

It boils down into two main parts:

- **[huff_structs](https://github.com/k-xlsx/huff-encoding/tree/dev/src/huff_structs)** -> structs used to represent a *Huffman Tree*
- **[file](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs)** -> methods to compress/decompress byte data or write/read compressed files.

### **[huff_structs](https://github.com/k-xlsx/huff-encoding/tree/dev/src/huff_structs)**

- **[HuffTree](https://github.com/k-xlsx/huff-encoding/blob/dev/src/huff_structs/tree.rs#L11)**
  - Represents a *Huffman Tree*
  - Can be built from bytes or *ByteFreqs*
- **[HuffBranch](https://github.com/k-xlsx/huff-encoding/blob/dev/src/huff_structs/branch.rs#L8)**
  - Represents a single branch in *HuffTree*.
  - Contains
    - Two children (*HuffBranches*)
    - It's position in parent *HuffBranch*
    - A *HuffLeaf*
- **[HuffLeaf](https://github.com/k-xlsx/huff-encoding/blob/dev/src/huff_structs/leaf.rs#L5)**
  - Represents the data stored in a *HuffBranch*
  - Contains:
    - The byte it's representing (can be *None* if it's a joint branch)
    - The stored frequency
    - It's own *HuffCode* in the *HuffTree*
- **[HuffCode](https://github.com/k-xlsx/huff-encoding/blob/dev/src/huff_structs/code.rs#L2)**
  - Represents the code of a *HuffBranch*
  - It can store up to 256 bits ([max size of a *Huffman code* is alphabetsize - 1](https://cs.stackexchange.com/questions/75542/maximum-size-of-huffman-codes-for-an-alphabet-containing-256-letters/75550#75550), so in my case its 256, as i'm using bytes for the alphabet, minus 1)
- **[ByteFreqs](https://github.com/k-xlsx/huff-encoding/blob/dev/src/huff_structs/freqs.rs#L7)**
  - Struct used to count the frequencies of each byte in the provided byte slice
  - Implements multithreading for faster counting

### **[file](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs)**

- **[compress](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs#L158)/[threaded_compress](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs#L183)**
  - Method accepting a byte slice as an argument and returning it compressed
  - Compressed bytes structure (*Big Endian*)
    - Byte containing the number of padding bits
      - First nibble -> header padding bits
      - Second nibble -> compressed contents padding bits
  - Header
    - 4 byte header length (in bytes)
    - *HuffTree* encoded in binary
  - Compressed data
- **[decompress](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs#L225)**
  - Method accepting compressed bytes slice and returning them decompressed.
  - **[WARNING]** Currently is kinda slow
- **[write_hfe](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs#L42)/[threaded_write_hfe](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs#L74)**
  - Method accepting
    - Destination file path (without file name)
    - Destination file name
    - Original file's extension (optional)
    - Original file's bytes
  - Compresses the original file's bytes and writes them to *dst_path/dst_filename*
  - hfe compressed file structure (*Big Endian*)
    - 255 bytes (just to be safe) storing the original file's extension
    - Byte containing the number of padding bits
      - First nibble -> header padding bits
      - Second nibble -> compressed contents padding bits
    - Header
      - 4 byte header length (in bytes)
      - HuffTree encoded in binary
    - Compressed data
- **[read_hfe](https://github.com/k-xlsx/huff-encoding/blob/dev/src/file.rs#L133)**
  - Method accepting the path of the compressed file and returning a *FileDecompressResult* containing
    - Original file's extension
    - Decompressed bytes

## Learn more about the Huffman Coding algorithm

Cool articles/videos about the *Huffman Coding* algorithm I found and learned from while working on this

- Articles
  - [Wikipedia](https://en.wikipedia.org/wiki/Huffman_coding)
  - [tutorialspoint](https://www.tutorialspoint.com/huffman-coding)
  - [Programiz](https://www.programiz.com/dsa/huffman-coding)
  - [GeeksforGeeks](https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/)
  - [Stack Exchange thread on *Huffman Tree* sizes](https://cs.stackexchange.com/questions/75542/maximum-size-of-huffman-codes-for-an-alphabet-containing-256-letters)
  - [*"Maximal codeword lengths in Huffman codes"* by Y.S.Abu-Mostafa and R.J.McEliece](https://www.sciencedirect.com/science/article/pii/S089812210000119X)
- Videos
  - [Tom Scott's The Basics](https://www.youtube.com/watch?v=JsTptu56GM8)
  - [Computerphile on *Huffman Trees*](https://www.youtube.com/watch?v=umTbivyJoiI)
  - [Computerphile on Compression](https://www.youtube.com/watch?v=Lto-ajuqW3w)
  - [Abdul Bari's video](https://www.youtube.com/watch?v=co4_ahEDCho)

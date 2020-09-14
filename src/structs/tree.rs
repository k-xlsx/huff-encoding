use std::{char, str};
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;

use bit_vec::BitVec;

use crate::structs::{HuffBranch, HuffLeaf, chars_to_freq};
use crate::structs::branch_heap::HuffBranchHeap;



/// Struct representing a Huffman Tree.
/// 
/// A HuffTree is comprised of HuffBranches, each having
/// 2 or 0 children, with root being the top one and 
/// every bottom one containing a char.
/// 
/// Can be grown from: 
/// ```
/// HashMap<char, usize>
/// ```
/// or 
/// ```
/// &str
/// ```
/// or even initialized empty and grown afterwards.
/// 
#[derive(Debug)]
pub struct HuffTree{
    root: Option<Rc<RefCell<HuffBranch>>>,
    char_codes: HashMap<char, BitVec>,
}

impl HuffTree{
    pub fn from(s: &str) -> HuffTree{
        //! Creates a HuffTree from:
        //! ```
        //! &str
        //! ```
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::from("Hello, World!");
        //! ```


        let mut huff_tree = HuffTree::new(None);
        huff_tree.grow(s);

        return huff_tree
    } 

    pub fn from_ctf(ctf: &HashMap<char, usize>) -> HuffTree{
        //! Creates a HuffTree from:
        //! ```
        //! HashMap<char, usize>
        //! ```
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::{HuffTree, get_chars_to_freq};
        //! 
        //! let foo = HuffTree::from_ctf(get_chars_to_freq("Hello, World!"));
        //! ```


        let mut huff_tree = HuffTree::new(None);
        huff_tree.grow_ctf(ctf);

        return huff_tree;
    }

    pub fn new(root: Option<Rc<RefCell<HuffBranch>>>) -> HuffTree{
        //! Initializes a HuffTree with the given root.
        //! 
        //! Can be grown later with .grow or .grow_ctf
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::new();
        //! foo.grow("Hello, World!");
        //! ```


        let huff_tree = HuffTree{
            root: root,
            char_codes: HashMap::new(),
        };

        return huff_tree;
    }

    pub fn coded_chars_from_bin(bin: &BitVec) -> Result<HashMap<BitVec, char>, &str>{
        //! Returns coded_chars read from a tree represented in binary
        //! (BitVec)
        //! 
        //! To get a tree as binary use as_bin.
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::from("abbccc");
        //! let bar = HuffTree::char_codes_from_bin(&foo.as_bin());
        //! 
        //! print!("{:?}", bar)
        //! // Prints something like:
        //! // {
        //! //      10: 'a',
        //! //      0: 'c',
        //! //      11: 'b',
        //! // }
        //! ```

        fn revert_branch_code(branch_code: &mut BitVec, prev_branch: bool){
            match prev_branch{
                // prev branch was joint -> you're its first child
                true =>{
                    branch_code.push(false);
                }
                // prev branch was char -> you're someones second child
                false =>{
                    // back up if prev char was last child of some branch
                    while branch_code.pop().unwrap(){}
                    branch_code.push(true);
                }
            }
        }

        // this whole thing is probably atrocious, but it works?

        
        let mut coded_chars: HashMap<BitVec, char> = HashMap::new();

        // current branch code and previous branch bit
        let mut branch_code = BitVec::new();
        let mut prev_branch = true;
    
        // flags indicating whether you're reading a char, and reading how many bits to read
        let mut read_char = false;
        let mut read_utf8_negative = false;

        // how many char bits you've read and how many you're supposed to
        let mut char_bit_counter = 0;
        let mut max_char_bits = 0;

        let mut char_bin = BitVec::new();

        let bin_iter = bin.iter().skip(match bin[0]{true => 1, false => 0});
        for b in bin_iter{
            match read_char{
                // read char
                true => {
                    char_bin.push(b);
                    char_bit_counter += 1;

                    match read_utf8_negative{
                        // read how many bits to read
                        true =>{
                            if b && char_bit_counter != 0{
                                max_char_bits += 8;
                            }
                            else if !b{
                                if max_char_bits == 0{
                                    max_char_bits += 8;
                                }
                                read_utf8_negative = false;
                            }
                        }
                        // just read bits
                        false =>{
                            // when read all char bits.
                            if char_bit_counter == max_char_bits{
                                // convert c_code String to u32 and then to char
                                let c_bytes = &char_bin.to_bytes();
                                let c = str::from_utf8(c_bytes);
                                match c{
                                    Err(_) => return Err("non utf-8 char in tree"),
                                    _ => (),
                                }
                                let c = c.unwrap().chars().next().unwrap();
                                
                                // set all flags and counters to start
                                read_char = false;
                                char_bit_counter = 0;
                                max_char_bits = 0;
                                char_bin = BitVec::new();
            
                                coded_chars.insert({
                                    revert_branch_code(&mut branch_code, prev_branch);
                                    branch_code.clone()
                                }, c);
            
                                // set yourself as prev_child
                                prev_branch = false;
                            }
                        }
                    }
                }
                // read branches
                false => {
                    match b{
                        // found a joint branch
                        true =>{
                            revert_branch_code(&mut branch_code, prev_branch);
    
                            // set yourself as prev child
                            prev_branch = true;
                        }
                        // found a char branch
                        false =>{
                        // start reading char when a char branch is found
                        read_char = true;
                        read_utf8_negative = true;
                        }
                    }
                }
            }
        }

        return Ok(coded_chars);
    }


    pub fn root(&self) -> Option<&Rc<RefCell<HuffBranch>>>{
        //! Returns the root of the tree.
        

        match self.root{
            Some(_) =>
                return self.root.as_ref(),
            None =>
                return None,
        }
    }

    pub fn char_codes(&self) -> &HashMap<char, BitVec>{
        //! Returns a HashMaps of chars with their
        //! corresponding Huffman codes.


        return &self.char_codes;
    }


    pub fn to_bin(&self) -> BitVec{
        //! Returns the tree represented in binary
        //! to be stored as a header to an encoded file:
        //! 
        //! 
        //! * 0 being a character branch (after a 0 you can expect an utf-8 encoded char.)
        //! * 1 being a joint branch.
        //! 
        //! To decode use:
        //! ```
        //! HuffTree::char_codes_from_bin(bin);
        //! ```
        //! 
        //! ---
        //! ## DOES NOT STORE FREQUENCIES.
        //! It's only meant to construct a same
        //! shaped tree for decoding a file.
        //! 
        //! ---
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::from("abbccc");
        //! 
        //! print!("{:?}", &foo.as_bit_vec()[..])
        //! // outputs:
        //! // 10000000000000000000000000011000111000000000000000000000000001100001000000000000000000000000001100010
        //! ```



        let mut bit_vec = BitVec::new();
        HuffTree::set_tree_as_bin(&mut bit_vec, self.root().unwrap().borrow());
        
        return bit_vec;
    }


    pub fn grow(&mut self, s: &str){
        //! Grows the tree from the given:HuffTree
        //! ```
        //! &str
        //! ```


        assert!(s.len() > 0, "string slice is empty");
        self.grow_ctf(&chars_to_freq(s));
    }

    pub fn grow_ctf(&mut self, ctf: &HashMap<char, usize>){
        //! Grows the tree from the given
        //! ```
        //! &HashMap<char, usize>
        //! ```


        assert!(ctf.len() > 0, "ctf is empty");


        let mut branch_heap = HuffBranchHeap::from(&ctf);

        while branch_heap.len() > 1{
            let mut min = branch_heap.pop_min();
            let mut next_min = branch_heap.pop_min();
            min.set_pos_in_parent(0);
            next_min.set_pos_in_parent(1);

            // initialize a joint branch and push it onto the heap
            let branch = HuffBranch::new(
                HuffLeaf::new(
                    None,
                    min.leaf().frequency() + next_min.leaf().frequency()
                ),
                Some([Rc::new(RefCell::new(min)), Rc::new(RefCell::new(next_min))])
            );
            branch_heap.push(branch);
        }

        // last branch in branch_heap is root
        let root = Some(Rc::new(RefCell::new(branch_heap.pop_min())));
        self.root = root;

        // set codes for all branches recursively
        HuffTree::set_codes_in_branches(self.root().unwrap().borrow_mut());

        // set char_codes recursively
        let mut char_codes: HashMap<char, BitVec> = HashMap::new();
        self.set_char_codes(&mut char_codes, self.root().unwrap().borrow());
        self.char_codes = char_codes;
    }


    fn set_char_codes(&self, char_codes: &mut HashMap<char, BitVec>, root: Ref<HuffBranch>){
        //! Recursively insert chars to codes into the given char_codes HashMap<char, BitVec>


        let root = root;
        let children = root.children();

        match children{
            Some(_) =>{   
                for child in children.unwrap().iter(){
                    let branch = child.borrow();
                    let leaf = branch.leaf();
                    let c = leaf.character();
                    match c{
                        Some(_) =>{
                            char_codes.insert(c.unwrap(), leaf.code().unwrap().clone());
                        }
                        None =>{
                            self.set_char_codes(char_codes, child.borrow());
                        }
                    }
                }
            }
            None =>{
                char_codes.insert(root.leaf().character().unwrap(), {let mut b = BitVec::new(); b.push(false); b});
            }
        }

    }

    fn set_codes_in_branches(root: RefMut<HuffBranch>){
        //! Recursively set codes on every branch


        let root = root;
        let children = root.children();

        match children{
            Some(_) =>{
                let root_code = root.leaf().code();

                // set codes on children and call set_branch_codes on them
                for child in children.unwrap().iter(){
                    child.borrow_mut().set_code(root_code);
                    HuffTree::set_codes_in_branches(child.borrow_mut());
                }
            }
            None =>
                (),
        }
    }

    fn set_tree_as_bin(tree_bvec: &mut BitVec, root: Ref<HuffBranch>){
        //! Recursively push bits to the given BitVec
        //! depending on the branches you encounter:
        //! * 0 being a char branch (followed by a utf-8 encoded char)
        //! * 1 being a joint branch


        let root = root;
        let children = root.children();

        match children{
            // children -> joint branch
            Some(_) =>{
                // 1 means joint branch
                tree_bvec.push(true);

                // call set_bin on children
                for child in children.unwrap().iter(){
                    HuffTree::set_tree_as_bin(tree_bvec, child.borrow());
                }
            }
            // no children -> char branch
            None =>{
                // 0 means char branch
                tree_bvec.push(false);

                // convert stored char to utf-8 bin code and write it after the 0
                let c = root.leaf().character().unwrap();
                let mut c_buffer = vec![0; 4];
                c.encode_utf8(&mut c_buffer);

                let mut cleaned_c_buffer: Vec<u8> = Vec::new();
                for (i, byte) in c_buffer.iter().enumerate(){
                    if *byte != 0 || i == 0{
                        cleaned_c_buffer.push(*byte)
                    }
                }
                
                let c_bvec = BitVec::from_bytes(&cleaned_c_buffer);
                for b in c_bvec{
                    tree_bvec.push(b);
                }
            }
        }
    }
}

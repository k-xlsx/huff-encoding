use bitvec::prelude::{BitVec, LocalBits};

use std::{
    collections::HashMap,
    cell::{RefCell, Ref, RefMut}, 
};

use crate::{
    huff_structs::branch_heap::HuffBranchHeap,
    {HuffBranch, HuffLeaf, HuffCode, ByteFreqs},
};



/// Struct representing a Huffman Tree.
/// 
/// A HuffTree is comprised of HuffBranches, each having
/// 2 or 0 children, with root being the top one and 
/// every bottom one containing a byte.
/// 
/// Can be grown from: 
/// 
/// * HashMap<u8, usize>
///
/// or 
/// 
/// * &str
/// 
/// or even initialized empty and grown afterwards.
/// 
#[derive(Debug)]
pub struct HuffTree{
    root: Option<Box<RefCell<HuffBranch>>>,
    byte_codes: HashMap<u8, HuffCode>,
}

impl Default for HuffTree{
    fn default() -> Self{
        Self::new()
    }
}

impl HuffTree{
    /// Initialize the tree from given bytes
    /// 
    /// Threaded version is faster for bigger files (huff_encoding::HuffTree::threaded_from_bytes).
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::HuffTree;
    /// 
    /// let foo = HuffTree::from_bytes("bar".as_bytes());
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> HuffTree{
        let mut tree = HuffTree::new();
        tree.grow(&ByteFreqs::from_bytes(&bytes));
        tree
    }

    /// Initialize the tree from given bytes, but using
    /// multiple threads (It's faster for bigger files.).
    /// 
    /// Non-threaded version is faster for smaller files (huff_encoding::HuffTree::from_bytes).
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::HuffTree;
    /// 
    /// let foo = HuffTree::threaded_from_bytes("bar".as_bytes());
    /// ```
    pub fn threaded_from_bytes(bytes: &[u8]) -> HuffTree{
        let mut tree = HuffTree::new();
        tree.grow(&ByteFreqs::threaded_from_bytes(&bytes));
        tree
    }

    /// Initializes a HuffTree.
    /// 
    /// Can be grown later with .grow or .grow_ctf
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::{HuffTree, ByteFreqs};
    /// 
    /// let mut foo = HuffTree::new();
    /// foo.grow(&ByteFreqs::from_bytes(&"Hello, World!".as_bytes()));
    /// ```
    pub fn new() -> HuffTree{
        HuffTree{
            root: None,
            byte_codes: HashMap::default(),
        }
    }

    /// Returns coded_chars read from a tree represented in binary
    /// (BitVec)
    /// 
    /// To get a tree as binary use as_bin.
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::HuffTree;
    /// 
    /// let foo = HuffTree::from_bytes("abbccc".as_bytes());
    /// let bar = HuffTree::coded_bytes_from_bin(&foo.to_bin());
    /// 
    /// print!("{:?}", bar)
    /// // Prints something like:
    /// // {
    /// //      10: 97,
    /// //      0:  99,
    /// //      11: 98,
    /// // }
    /// ```
    pub fn coded_bytes_from_bin(bin: &BitVec<LocalBits, u8>) -> HashMap<HuffCode, u8>{
        fn revert_branch_code(branch_code: &mut HuffCode, prev_branch: bool){
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

        
        let mut coded_bytes: HashMap<HuffCode, u8> = HashMap::with_capacity(256);

        // current branch code and previous branch bit
        let mut branch_code = HuffCode::new();
        let mut prev_branch = true;
    
        let mut read_byte = false;
        let mut read_byte_counter = 0;

        let mut byte_bit_vec = BitVec::<LocalBits, u8>::with_capacity(8);

        let bin_iter = bin.iter().skip(match bin[0]{true => 1, false => 0});
        for bit in bin_iter{
            match read_byte{
                // read byte
                true => {
                    byte_bit_vec.push(*bit);
                    read_byte_counter += 1;

                    // when read all byte bits.
                    if read_byte_counter == 8{
                        let byte = &byte_bit_vec.into_vec()[0];
                                
                        // reset reading byte params
                        read_byte = false;
                        read_byte_counter = 0;
                        byte_bit_vec = BitVec::with_capacity(8);
            
                        coded_bytes.insert({
                            revert_branch_code(&mut branch_code, prev_branch);
                            branch_code.clone()
                        }, *byte);
            
                        // set yourself as prev_child
                        prev_branch = false;
                    }
                }
                // read branches
                false => {
                    match bit{
                        // found a joint branch
                        true =>{
                            revert_branch_code(&mut branch_code, prev_branch);
    
                            // set yourself as prev child
                            prev_branch = true;
                        }
                        // found a byte branch
                        false =>{
                        // start reading byte when a byte branch is found
                        read_byte = true;
                        }
                    }
                }
            }
        }

        coded_bytes
    }


    /// Returns the root of the tree.
    pub fn root(&self) -> Option<&Box<RefCell<HuffBranch>>>{
        match self.root{
            Some(_) =>
                self.root.as_ref(),
            None =>
                None,
        }
    }

    /// Returns reference to a HashMap of bytes with their
    /// corresponding Huffman codes.
    pub fn byte_codes(&self) -> &HashMap<u8, HuffCode>{
        &self.byte_codes
    }


    /// Returns the tree represented in binary
    /// to be stored as a header to an encoded file:
    /// 
    /// 
    /// * 0 being a byte branch (after a 0 you can expect a byte of data)
    /// * 1 being a joint branch.
    /// 
    /// To decode use:
    /// 
    /// * HuffTree::char_codes_from_bin(bin);
    /// 
    /// ---
    /// ## DOES NOT STORE FREQUENCIES.
    /// It's only meant to construct a same
    /// shaped tree for decoding a file.
    /// 
    /// ---
    /// 
    /// # Examples
    /// ---
    /// ```
    /// use huff_encoding::HuffTree;
    /// 
    /// let foo = HuffTree::from_bytes("abbccc".as_bytes());
    /// 
    /// print!("{:?}", &foo.to_bin())
    /// // outputs:
    /// // 10011000111001100001001100010
    /// ```
    pub fn to_bin(&self) -> BitVec<LocalBits, u8>{
        /// Recursively push bits to the given BitVec
        /// depending on the branches you encounter:
        /// * 0 being a byte branch (followed by a byte of data, duh)
        /// * 1 being a joint branch
        fn set_tree_as_bin(tree_bvec: &mut BitVec<LocalBits, u8>, root: Ref<HuffBranch>){
            let root = root;
            let children = root.children();

            match children{
                // children -> joint branch
                Some(_) =>{
                    // 1 means joint branch
                    tree_bvec.push(true);

                    // call set_bin on children
                    for child in children.unwrap().iter(){
                        set_tree_as_bin(tree_bvec, child.borrow());
                    }
                }
                // no children -> byte branch
                None =>{
                    // 0 means byte branch
                    tree_bvec.push(false);

                    let byte = root.leaf().byte().unwrap();
                    
                    let byte_bvec = BitVec::<LocalBits, u8>::from_vec([byte].to_vec());
                    for byte in byte_bvec{
                        tree_bvec.push(byte);
                    }
                }
            }
        }

        let mut bit_vec: BitVec<LocalBits, u8> = BitVec::new();
        set_tree_as_bin(&mut bit_vec, self.root().unwrap().borrow());
        
        bit_vec
    }


    /// Grows the tree from the given
    /// * &ByteFreqs
    pub fn grow(&mut self, byte_freqs: &ByteFreqs){
        assert!(!byte_freqs.is_empty(), "byte_freqs is empty");


        let mut branch_heap = HuffBranchHeap::from_byte_freqs(&byte_freqs);

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
                Some([Box::new(RefCell::new(min)), Box::new(RefCell::new(next_min))])
            );
            branch_heap.push(branch);
        }



        // last branch in branch_heap is root
        let root = Some(Box::new(RefCell::new(branch_heap.pop_min())));
        self.root = root;

        // set codes for all branches recursively
        HuffTree::set_codes_in_branches(self.root().unwrap().borrow_mut());

        // set byte_codes recursively
        let mut byte_codes: HashMap<u8, HuffCode> = HashMap::default();
        self.set_byte_codes(&mut byte_codes, self.root().unwrap().borrow());
        self.byte_codes = byte_codes;
    }


    /// Recursively insert bytes to codes into the given byte_codes HashMap<u8, BitVec>
    fn set_byte_codes(&self, byte_codes: &mut HashMap<u8, HuffCode>, root: Ref<HuffBranch>){
        let root = root;
        let children = root.children();

        match children{
            Some(_) =>{   
                for child in children.unwrap().iter(){
                    let branch = child.borrow();
                    let leaf = branch.leaf();
                    let byte = leaf.byte();
                    match byte{
                        Some(_) =>{
                            byte_codes.insert(byte.unwrap(), leaf.code().unwrap().clone());
                        }
                        None =>{
                            self.set_byte_codes(byte_codes, child.borrow());
                        }
                    }
                }
            }
            None =>{
                byte_codes.insert(root.leaf().byte().unwrap(), {let mut b = HuffCode::new(); b.push(false); b});
            }
        }

    }

    /// Recursively set codes on every branch
    fn set_codes_in_branches(root: RefMut<HuffBranch>){
        let root = root;

        if let Some(children) = root.children(){
            let root_code = root.leaf().code();

            // set codes on children and call set_branch_codes on them
            for child in children.iter(){
                child.borrow_mut().set_code(root_code);
                HuffTree::set_codes_in_branches(child.borrow_mut());
            }
        }
    }
}

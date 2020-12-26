use bitvec::prelude::*;

use std::{
    cell::{RefCell, Ref, RefMut},
    collections::{hash_map::RandomState, HashMap},
    hash::BuildHasher,
};

use super::{
    HuffBranch, HuffLeaf, HuffLetter, HuffLetterBitStore,
    branch_heap::HuffBranchHeap
};
use crate::{
    utils::size_of_bits,
    freqs::Freq,
};



#[derive(Debug, Clone)]
pub struct HuffTree<L: HuffLetter>{
    root: HuffBranch<L>,
}

impl<L: HuffLetter> HuffTree<L>{
    pub fn from_freqs<F: Freq<L>>(byte_freqs: F) -> Self{
        assert!(!byte_freqs.is_empty(), "cannot create tree from 0 bytes");

        let mut branch_heap = HuffBranchHeap::from_byte_freqs(byte_freqs);

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
        let root = RefCell::new(branch_heap.pop_min());
        // set codes for all branches recursively
        HuffTree::set_codes_in_branches(root.borrow_mut());

        HuffTree{
            root: root.into_inner(),
        }
    }

    pub fn root(&self) -> &HuffBranch<L>{
        &self.root
    }

    pub fn codes(&self) -> HashMap<L, BitVec<Msb0, u8>>{
        self.codes_with_hasher(RandomState::default())
    }

    pub fn codes_with_hasher<S: BuildHasher>(&self, hash_builder: S) -> HashMap<L, BitVec<Msb0, u8>, S>{
        /// Recursively insert bytes to codes into the given byte_codes HashMap<u8, BitVec>
        fn set_codes<L: HuffLetter, S: BuildHasher>(byte_codes: &mut HashMap<L, BitVec<Msb0, u8>, S>, root: Ref<HuffBranch<L>>){
            let root = root;
            let children = root.children();

            match children{
                Some(children) =>{   
                    for child in children.iter(){
                        let branch = child.borrow();
                        let leaf = branch.leaf();
                        let letter = leaf.letter();
                        match letter{
                            Some(letter) =>{
                                byte_codes.insert(letter.clone(), leaf.code().unwrap().clone());
                            }
                            None =>{
                                set_codes(byte_codes, child.borrow());
                            }
                        }
                    }
                }
                None =>{
                    byte_codes.insert(root.leaf().letter().unwrap().clone(), {let mut b = BitVec::new(); b.push(false); b});
                }
            }

        }
        let mut codes = HashMap::with_hasher(hash_builder);
        set_codes(&mut codes, self.root().children().unwrap()[0].borrow());
        set_codes(&mut codes, self.root().children().unwrap()[1].borrow());
        codes
    }

    fn set_codes_in_branches(root: RefMut<HuffBranch<L>>){
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

impl<L: 'static + HuffLetterBitStore> HuffTree<L>{
    pub fn from_bits(mut bits: BitVec) -> Self{
        /// Recursively reads branches and their children from the given bits
        /// When finding a 1 -> recurses to get children,
        /// and when a 0 -> ends recursion returning a letter branch
        fn read_branches_from_bits<L: HuffLetterBitStore>(bits: &mut BitVec, pos_in_parent: bool) -> HuffBranch<L>{
            // remove first bit, if its 1 -> joint branch
            if bits.drain(..1).next().unwrap(){
                // create joint branch, recurse to get children
                let mut branch = HuffBranch::new(
                    HuffLeaf::new(None, 0),
                    Some([
                        Box::new(RefCell::new(read_branches_from_bits(bits, false))), 
                        Box::new(RefCell::new(read_branches_from_bits(bits, true)))
                    ])
                );
                branch.set_pos_in_parent(pos_in_parent as u8);
                branch
            }
            // if it's 0 -> byte branch
            else{
                let mut letter = BitVec::<Msb0, L>::new();
                // read the letter, removing it's bits
                for bit in bits.drain(..size_of_bits::<L>()){
                    letter.push(bit);
                }
                // create byte branch (no children)
                let mut branch = HuffBranch::new(
                    HuffLeaf::new(Some(letter.into_vec()[0]), 0),
                    None,
                );
                branch.set_pos_in_parent(pos_in_parent as u8);
                branch
            }
        }
        // recurse to create root, and set codes for all branches
        let root = RefCell::new(read_branches_from_bits(&mut bits, false));
        HuffTree::set_codes_in_branches(root.borrow_mut());

        HuffTree{
            root: root.into_inner(),
        }
    }

    pub fn as_bits(&self) -> BitVec{
        /// Recursively push bits to the given BitVec
        /// depending on the branches you encounter:
        /// * 0 being a letter branch (followed by a letter encoded in binary)
        /// * 1 being a joint branch
        fn set_tree_as_bin<L: 'static + HuffLetterBitStore>(tree_bitvec: &mut BitVec, root: Ref<HuffBranch<L>>){
            let root = root;
            let children = root.children();

            match children{
                // children -> joint branch
                Some(children) =>{
                    // 1 means joint branch
                    tree_bitvec.push(true);

                    // call set_bin on children
                    for child in children.iter(){
                        set_tree_as_bin(tree_bitvec, child.borrow());
                    }
                }
                // no children -> letter branch
                None =>{
                    // 0 means letter branch
                    tree_bitvec.push(false);
                    for bit in BitVec::<Msb0, L>::from_vec(vec![*root.leaf().letter().unwrap()]){
                        tree_bitvec.push(bit);
                    }
                }
            }
        }

        let mut bit_vec = BitVec::new();
        bit_vec.push(true);
        set_tree_as_bin(&mut bit_vec, self.root().children().unwrap()[0].borrow());
        set_tree_as_bin(&mut bit_vec, self.root().children().unwrap()[1].borrow());
        bit_vec
    }
}

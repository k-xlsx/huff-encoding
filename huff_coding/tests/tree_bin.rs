use huff_coding::{
    prelude::*,
    bitvec::prelude::*,
};



#[test]
fn tree_from_bin(){
    let tree = HuffTree::from_weights(ByteWeights::from_bytes(b"Mongo...
    a great barbarian from the north seeking to conquer new lands for his kingdom.
    Mysterio the Magnificent...
    a powerful wizard questing for the secret of immortality."));
    let tree_from_bin = HuffTree::try_from_bin(tree.as_bin()).unwrap();
    assert_eq!(tree_from_bin.read_codes(), tree.read_codes());
}


#[test]
#[should_panic]
fn tree_bin_invalid_type(){
    let tree = HuffTree::from_weights(ByteWeights::from_bytes(b"Erutan Revol...
    Elven Warden sworn to protect Nature, with his own life if need should arise.
    Baron von Tarkin...
    Master of Death waging war against the forces of Life."));
    let tree_bin = tree.as_bin();
    HuffTree::<u128>::try_from_bin(tree_bin).unwrap();
}

#[test]
#[should_panic]
fn tree_bin_invalid_vec(){
    HuffTree::<u8>::try_from_bin(BitVec::new()).unwrap();
}

use huff_coding::{
    prelude::*,
    bitvec::prelude::*,
};

use std::collections::HashMap;



#[test]
fn tree_normal_init(){
    let weights = {
        let mut h = HashMap::new();
        h.insert("Spazz", 5);
        h.insert("Maticus...", 9);
        h.insert("a young King", 12);
        h.insert("on a mad", 13);
        h.insert("quest", 16);
        h.insert("to rule the world.", 45);
        h
    };
    let tree = HuffTree::from_weights(weights);
    let codes = tree.letter_codes();

    assert_eq!(
        codes.get("Spazz"), 
        Some(&bitvec![Msb0, u8; 1, 1, 0, 0])
    );
    assert_eq!(
        codes.get("Maticus..."), 
        Some(&bitvec![Msb0, u8; 1, 1, 0, 1])
    );
    assert_eq!(
        codes.get("a young King"),
        Some(&bitvec![Msb0, u8; 1, 0, 0])
    );
    assert_eq!(
        codes.get("on a mad"), 
        Some(&bitvec![Msb0, u8; 1, 0, 1])
    );
    assert_eq!(
        codes.get("quest"), 
        Some(&bitvec![Msb0, u8; 1, 1, 1])
    );
    assert_eq!(
        codes.get("to rule the world."),
        Some(&bitvec![Msb0, u8; 0])
    );
}

#[test]
fn tree_single_branch(){
    let w = {
        let mut h = HashMap::new();
        h.insert(-12, 78);
        h
    };
    let tree = HuffTree::from_weights(w);

    // letter branch -12 is root
    assert_eq!(tree.root().leaf().letter(), Some(&-12));

    // code equals 0
    assert_eq!(tree.root().leaf().code().unwrap()[0], false);
    assert_eq!(tree.root().leaf().code().unwrap().get(1), None);
}

#[test]
#[should_panic(expected = "provided empty weights")]
fn tree_invalid_weights(){
    HuffTree::from_weights(HashMap::<char, usize>::new());
}
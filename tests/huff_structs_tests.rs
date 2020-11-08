use huff_encoding::*;

#[test]
fn struct_equality(){
    // leaf
    assert_eq!(
        HuffLeaf::new(None, 2),
         HuffLeaf::new(None, 2)
    );

    // branch
    assert_eq!(
        HuffBranch::new(
            HuffLeaf::new(None, 2),
            None
            ),
            HuffBranch::new(
                HuffLeaf::new(None, 2),
                None
            ),
    );

    // code
    let mut codesa = HuffCode::new();
    let mut codesb = HuffCode::new();

    codesa.push(true);
    codesb.push(false);

    assert!(codesa != codesb);
}

#[test]
fn code_interfacing(){
    // getting
    let mut codes = HuffCode::new();
    for i in 0..8{
        if (i % 2) == 0{
            codes.push(true);
        }
        else{
            codes.push(false);
        }
    }
    assert_eq!(codes.get(7), Some(false));
    assert_eq!(codes.get(2), Some(true));
    assert_eq!(codes.get(31), None);

    // pushing and popping
    let mut codea = huff_encoding::HuffCode::new();
    for _ in 0..65{
        codea.push(true);
    }
    codea.pop();
    let mut codeb = huff_encoding::HuffCode::new();
    for _ in 0..64{
        codeb.push(true);
    }

    assert_eq!(codea, codeb);
}

#[test]
fn code_iter(){
    let mut codes = HuffCode::new();

    for i in 0..8{
        if (i % 2) == 0{
            codes.push(true);
        }
        else{
            codes.push(false);
        }
    }

    for (i, b) in codes.iter().enumerate(){
        if (i % 2) == 0{
            assert_eq!(b, true);
        }
        else{
            assert_eq!(b, false);
        }
    }
}

#[test]
fn freqs_iter(){
    let byte_freqs = ByteFreqs::from_bytes(&[255, 255, 255, 255, 255]);

    let mut len = 0;
    for (b, f) in byte_freqs.iter(){
        len += 1;
        println!("{:?}", (b, f))
    }
    assert_eq!(len, byte_freqs.len());
}

#[test]
fn tree_bin_codes(){
    let tree = HuffTree::from_bytes("Spazz Maticus...
    a young King on a mad quest to rule the world.".as_bytes());
    let tree_bin = tree.to_bin();

    let bin_byte_codes = HuffTree::coded_bytes_from_bin(&tree_bin);
    for (code, byte) in bin_byte_codes{
        assert_eq!(tree.byte_codes().get(&byte).unwrap(), &{let mut hc = HuffCode::new(); for b in code.iter(){hc.push(b)}; hc}); 
    }
}

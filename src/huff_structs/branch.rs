#![allow(dead_code)]


use std::rc::Rc;
use crate::huff_structs::HuffLeaf;



#[derive(Debug, Clone)]
pub struct HuffBranch{
    leaf: HuffLeaf,

    parent: Option<Rc<HuffBranch>>,
    pos_in_parent: Option<u8>,
    right: Option<Rc<HuffBranch>>,
}

impl HuffBranch{
    pub fn new(leaf: HuffLeaf, parent: Option<Rc<HuffBranch>>, pos_in_parent: Option<u8>, left: Option<Rc<HuffBranch>>, right: Option<Rc<HuffBranch>>) -> HuffBranch{

    pub fn new(leaf: HuffLeaf, children: [Option<Rc<HuffBranch>>; 2]) -> HuffBranch{

        let huff_branch = HuffBranch{
            leaf: leaf,

            parent: None,
            pos_in_parent: None,
            children: children
        };

        return huff_branch;
    }


    pub fn leaf(&self) -> &HuffLeaf{
        return &self.leaf;
    }

    pub fn parent(&self) -> Option<&Rc<HuffBranch>>{
        return self.parent.as_ref();
    }

    pub fn pos_in_parent(&self) -> Option<u8>{
        return self.pos_in_parent
    }

    pub fn children(&self) -> [Option<&Rc<HuffBranch>>; 2]{
        return [self.children[0].as_ref(), self.children[1].as_ref()]
    }


    pub fn set_parent(&mut self, parent: Rc<HuffBranch>, pos_in_parent: u8){
        assert!(pos_in_parent <= 1, "pos_in_parent must be binary");

        self.parent = Some(parent);
        self.pos_in_parent = Some(pos_in_parent);
    }

    pub fn set_children(&mut self, children: [Option<Rc<HuffBranch>>; 2]){
        self.children = children
    }

    pub fn set_leaf_code(&mut self){
        let mut code = String::new();
        match self.parent(){
            Some(_) => {
                let parent_code = self.parent().unwrap().leaf().code();
                match parent_code{
                    Some(_) => {
                        code.push_str(&parent_code.as_ref().unwrap())
                    }
                    None =>
                        (),
                }

                code.push(match self.pos_in_parent().unwrap(){
                        0 => '0',
                        1 => '1',
                        _ => panic!("pos_in_parent not binary")
                });
                
                code = code.chars().rev().collect();
                let code = &code[..];

                self.leaf.set_code(code)
            }
            None =>
                (),
        }

    }
}


#![allow(dead_code)]


use std::rc::Rc;
use crate::huff_structs::HuffLeaf;



#[derive(Debug, Clone)]
pub struct HuffBranch{
    leaf: HuffLeaf,

    pos_in_parent: Option<u8>,
    children: [Option<Rc<HuffBranch>>; 2]
}

impl HuffBranch{
    pub fn new(leaf: HuffLeaf, children: [Option<Rc<HuffBranch>>; 2]) -> HuffBranch{

        let huff_branch = HuffBranch{
            leaf: leaf,

            pos_in_parent: None,
            children: children
        };

        return huff_branch;
    }


    pub fn leaf(&self) -> &HuffLeaf{
        return &self.leaf;
    }

    pub fn mut_leaf(&mut self) -> &mut HuffLeaf{
        return &mut self.leaf;
    }

    pub fn pos_in_parent(&self) -> Option<u8>{
        return self.pos_in_parent
    }

    pub fn children(&self) -> [Option<&Rc<HuffBranch>>; 2]{
        return [self.children[0].as_ref(), self.children[1].as_ref()]
    }


    pub fn set_pos_in_parent(&mut self, pos_in_parent: u8){
        self.pos_in_parent = Some(pos_in_parent);
    } 

    pub fn set_leaf_code(&mut self, parent_code: Option<&String>){
        let mut code = String::new();

        match self.pos_in_parent(){
            Some(_) =>{        
                match parent_code{
                    Some(_) =>{
                        code.push_str(&parent_code.unwrap().chars().rev().collect::<String>());
                    }
                    None =>
                        (),
                }
                match self.pos_in_parent().unwrap(){
                    0 =>
                        code.push('0'),
                    1 =>
                        code.push('1'),
                    _ =>
                        panic!("pos_in_parent not binary"),
                }

                code = code.chars().rev().collect::<String>();
                self.mut_leaf().set_code(&code);
            }
            None =>
                (),
        }
    }
}


pub use treebin::HuffTreeBin;
pub use code::HuffCode;

// TODO: docs
// TODO: maybe a macro to quickly create HuffCodes

macro_rules! bitvec_wrapper_impl{
    {$name:ident$(<$generic:ident: $trait:ident>)?} => {
        use bitvec::{
            prelude::{BitVec, Msb0},
            vec::Drain,
        };

        use core::ops::RangeBounds;



        #[derive(Clone, PartialEq, Eq)]
        pub struct $name<$($generic: $trait)?>{
            storage: BitVec<Msb0, u8>,
            $(_typebind: std::marker::PhantomData<$generic>)?
        }

        impl<$($generic: $trait)?> std::fmt::Display for $name<$($generic)?>{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
                let mut bits: String = String::new();
                for bit in self.iter(){
                    bits.extend(format!("{}", bit as u8).chars());
                }
                f.write_str(&bits)
            }
        }

        impl<$($generic: $trait)?> std::fmt::Debug for $name<$($generic)?>{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
                let mut bits: String = String::new();
                for bit in self.iter(){
                    bits.extend(format!("{}", bit as u8).chars());
                }
                f.write_str(&bits)
            }
        }

        impl<$($generic: $trait)?> $name<$($generic)?>{
            pub fn new() -> Self{
                Self{
                    storage: BitVec::new(),
                    $(
                    _typebind: {
                        let foo: std::marker::PhantomData<$generic> = std::marker::PhantomData; 
                        foo
                    }
                    )?
                }
            }
        
            pub fn from_vec(vec: Vec<u8>) -> Self{
                Self{
                    storage: BitVec::from_vec(vec),
                    $(
                    _typebind: {
                        let foo: std::marker::PhantomData<$generic> = std::marker::PhantomData; 
                        foo
                    }
                    )?
                }
            }

            pub fn into_vec(self) -> Vec<u8>{
                self.storage.into_vec()
            }

            pub fn storage(&self) -> &BitVec<Msb0, u8>{
                &self.storage
            }
        
            pub fn len(&self) -> usize{
                self.storage.len()
            }

            pub fn is_empty(&self) -> bool{
                self.storage.is_empty()
            }

            pub fn first(&self) -> Option<bool>{
                Some(*self.storage.first().as_deref()?)
            }

            pub fn get(&self, index: usize) -> Option<bool>{
                Some(*self.storage.get(index).as_deref()?)
            }
            
            pub fn push(&mut self, bit: bool){
                self.storage.push(bit);
            }

            pub fn pop(&mut self) -> Option<bool>{
                self.storage.pop()
            }
        
            pub fn clear(&mut self){
                self.storage.clear();
            }

            pub fn iter<'a>(&self) -> Iter{
                self.into_iter()
            }

            pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, Msb0, u8>{
                self.storage.drain(range)
            }
        }

        pub struct IntoIter{
            iter: bitvec::vec::IntoIter<Msb0, u8>
        }

        impl Iterator for IntoIter{
            type Item = bool;
            
            fn next(&mut self) -> Option<Self::Item>{
                self.iter.next()
            }
        }

        impl<$($generic: $trait)?> IntoIterator for $name<$($generic)?>{
            type Item = bool;
            type IntoIter = IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter{
                    iter: self.storage.into_iter()
                }
            }
        }

        pub struct Iter<'a>{
            iter: bitvec::slice::Iter<'a, Msb0, u8>
        }
        
        impl<'a> Iterator for Iter<'a>{
            type Item = bool;

            fn next(&mut self) -> Option<Self::Item>{
                Some(*self.iter.next()?)
            }
        }

        impl<'a, $($generic: $trait)?> IntoIterator for &'a$name<$($generic)?>{
            type Item = bool;
            type IntoIter = Iter<'a>;

            fn into_iter(self) -> Self::IntoIter{
                Self::IntoIter{
                    iter: self.storage.iter()
                }
            }
        }
    };
}

mod treebin{
    use super::super::HuffLetterAsBytes;
    bitvec_wrapper_impl!{HuffTreeBin<L: HuffLetterAsBytes>}
}

mod code{
    bitvec_wrapper_impl!{HuffCode}
}

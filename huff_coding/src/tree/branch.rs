use super::{HuffLeaf, HuffLetter};
use crate::bitvec::prelude::*;

use std::{
    cell::RefCell, 
    cmp::Ordering,
};



/// Struct representing a branch in a ```HuffTree```
/// 
/// Stores:
/// * ```leaf: HuffLeaf<L>```
///  * a struct containing the branch's letter(of type ```L```), 
/// frequency & code
/// * ```children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>```
///  * references to its child ```HuffBranch```es
///  * ```None``` if has no children
/// 
/// # Examples
/// ---
/// Initializing a branch "by hand":
/// ```
/// use huff_coding::prelude::{HuffBranch, HuffLeaf};
/// use std::cell::RefCell;
/// 
/// let foo = HuffBranch::new(
///     HuffLeaf::new(
///         Some(0xaa),
///         0
///     ),
///     Some([
///         Box::new(RefCell::new(HuffBranch::new(
///             HuffLeaf::new(
///                 Some(0xbb),
///                 0
///             ),
///             None
///         ))),
///         Box::new(RefCell::new(HuffBranch::new(
///             HuffLeaf::new(
///                 Some(0xcc),
///                 0
///             ),
///             None
///         ))),
///     ])
/// );
/// ```
/// Setting the children after initialization:
/// ```
/// use huff_coding::prelude::{HuffBranch, HuffLeaf};
/// use std::cell::RefCell;
/// 
/// let mut foo = HuffBranch::new(
///     HuffLeaf::new(
///         Some("yś"),
///         12
///     ),
///     None
/// );
/// 
/// foo.set_children(
///     Some([
///         Box::new(RefCell::new(HuffBranch::new(
///             HuffLeaf::new(
///                 Some("yś"),
///                 2
///             ),
///             None
///         ))),
///         Box::new(RefCell::new(HuffBranch::new(
///             HuffLeaf::new(
///                 Some("omamale"),
///                 10
///             ),
///             None
///         )))
///     ])
/// );
/// ```
/// Comparing different branches:
/// ```
/// use huff_coding::prelude::{HuffBranch, HuffLeaf};
/// 
/// let foo = HuffBranch::new(
///     HuffLeaf::new(
///         Some('t'),
///         2137
///     ),
///     None
/// );
/// 
///  let bar = HuffBranch::new(
///     HuffLeaf::new(
///         Some('x'),
///         144
///     ),
///     None
/// );
/// 
/// let foobar = HuffBranch::new(
///     HuffLeaf::new(
///         Some('t'),
///         144
///     ),
///     None
/// );
/// 
/// assert!(foo > bar);
/// assert!(bar == foobar);
/// assert!(foobar != foo);
/// ```
#[derive(Debug, Clone, Eq)]
pub struct HuffBranch<L: HuffLetter>{
    leaf: HuffLeaf<L>,
    children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>
}

impl<L: HuffLetter> Ord for HuffBranch<L>{
    fn cmp(&self, other: &Self) -> Ordering {
        self.leaf().cmp(other.leaf())
    }
}

impl<L: HuffLetter> PartialOrd for HuffBranch<L>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: HuffLetter> PartialEq for HuffBranch<L>{
    fn eq(&self, other: &Self) -> bool {
        self.leaf() == other.leaf()
    }
}

impl<L: HuffLetter> HuffBranch<L>{
    /// Initialize a new ```HuffBranch<L>``` with the given leaf and children
    pub fn new(leaf: HuffLeaf<L>, children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>) -> Self{
        HuffBranch{
            leaf,
            children,
        }
    }

    /// Returns a reference to the ```HuffLeaf``` containing the branch's
    /// letter, frequency and code.
    pub fn leaf(&self) -> &HuffLeaf<L>{
        &self.leaf
    }

    /// Returns a reference to the children of the ```HuffBranch```:
    /// 
    /// ```&[Box<RefCell<HuffBranch<L>>>; 2]``` or 
    /// ```None``` if has no children
    pub fn children(&self) -> Option<&[Box<RefCell<HuffBranch<L>>>; 2]>{
        match self.children{
            None => 
                None,
            Some(_) => {
                self.children.as_ref()
            }
        }
    }

    /// Returns true if the branch has children
    pub fn has_children(&self) -> bool{
        self.children.is_some()
    }

    /// Setter for ```children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>```
    pub fn set_children(&mut self, children: Option<[Box<RefCell<HuffBranch<L>>>; 2]>){
        self.children = children;
    }

    /// Setter for the leaf's code 
    pub fn set_code(&mut self, code: BitVec<Msb0, u8>){
        self.leaf.set_code(code);
    }
}


// TODO: figure out how to export this from huff_coding::tree::branch
#[macro_export]
macro_rules! huff_branch {
    ($freq:expr, [$child1:expr, $child2:expr]) => {
        HuffBranch::new(
            HuffLeaf::new(None, $freq),
            Some([
                Box::new(std::cell::RefCell::new(
                    $child1
                )), 
                Box::new(std::cell::RefCell::new(
                    $child2
                ))
            ])
        );
    };
    ($lett:expr, $freq: expr) => {
        HuffBranch::new(
            HuffLeaf::new(Some($lett), $freq),
            None
        );
    };
}

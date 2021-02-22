pub(crate) mod includes;
use includes::*;

type Node<T> = Rc<RefCell<TreeNode<T>>>;

pub mod treenode;
pub mod traversal;
pub mod operations;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T> where T: Copy + Clone + Debug {
    root: Option<Node<T>>,
}

impl<T> Tree<T> where T: Copy + Clone + Debug {
    pub(in crate::tree) fn new() -> Self {
        Self { root: None }
    }
}

pub mod nodebuilder;
pub mod treebuilder;

pub(crate) mod includes;
use includes::*;

type Node<T> = Rc<RefCell<TreeNode<T>>>;

pub(crate) mod treenode;
pub(crate) mod traversal;

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

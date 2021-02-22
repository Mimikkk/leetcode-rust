use crate::tree::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> where T: Copy + Clone + Debug {
    pub val: T,
    pub left: Option<Node<T>>,
    pub right: Option<Node<T>>,
}

impl<T> TreeNode<T> where T: Copy + Clone + Debug {
    pub(crate) fn new(val: T) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

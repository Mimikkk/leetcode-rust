use crate::data_structures::tree::{Debug, Node, RefCell, Rc};
use crate::data_structures::tree::treenode::TreeNode;
pub struct NodeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
    node: TreeNode<T>
}

impl<T> NodeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
    pub fn new(val: T) -> Self { NodeBuilder { node: TreeNode::new(val) } }

    pub fn with_left(mut self, node: Option<Node<T>>) -> Self {
        self.node.left = node;
        self
    }

    pub fn with_right(mut self, node: Option<Node<T>>) -> Self {
        self.node.right = node;
        self
    }

    pub fn build(self) -> Option<Node<T>> {
        Some(Rc::new(RefCell::new(self.node)))
    }

    pub fn default() -> Option<Node<T>> where T: Default {
        Self::new(T::default()).build()
    }

    pub fn leaf(value: T) -> Option<Node<T>> { NodeBuilder::new(value).build() }
}

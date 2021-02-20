use crate::{
    tree::{Debug, Node, RefCell, Rc},
    tree::treenode::TreeNode,
};
pub struct NodeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
    node: TreeNode<T>
}

impl<T> NodeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
    pub fn new(val: T) -> Self { NodeBuilder { node: TreeNode::new(val) } }

    pub fn with_left(mut self, node: Node<T>) -> Self {
        self.node.left = Some(node);
        self
    }

    pub fn with_right(mut self, node: Node<T>) -> Self {
        self.node.right = Some(node);
        self
    }

    pub fn build(self) -> Node<T> {
        Rc::new(RefCell::new(self.node))
    }

    pub fn default() -> Node<T> where T: Default {
        Self::new(T::default()).build()
    }

    pub fn leaf(value: T) -> Node<T> { NodeBuilder::new(value).build() }
}

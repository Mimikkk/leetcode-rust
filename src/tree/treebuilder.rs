use crate::tree::*;
use crate::tree::nodebuilder::NodeBuilder;

pub struct TreeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
    tree: Tree<T>
}

impl<T> TreeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
    pub fn empty() -> Tree<T> { Tree::new() }

    pub fn new() -> Self { TreeBuilder { tree: Self::empty() } }

    pub fn with_root(mut self, node: Option<Node<T>>) -> Self {
        self.tree.root = node;
        self
    }

    pub fn build(self) -> Tree<T> { self.tree }


    pub fn from_json(_path: &str) -> Tree<T> { unimplemented!() }
}

impl TreeBuilder<i32> {
    pub fn example_i32() -> Tree<i32> {
        TreeBuilder::new().with_root(
            NodeBuilder::new(5).with_left(
                NodeBuilder::new(3).with_left(
                    NodeBuilder::leaf(1)
                ).with_right(
                    NodeBuilder::leaf(4)
                ).build()
            ).with_right(
                NodeBuilder::leaf(8)
            ).build()
        ).build()
    }
}

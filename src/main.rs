use rand::Rng;
use std::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
pub mod tree {
    pub(crate) use std::rc::Rc;
    pub(crate) use std::cell::{RefCell, Ref};
    pub(crate) use std::borrow::{Borrow, BorrowMut};
    pub(crate) use std::collections::VecDeque;
    pub(crate) use std::convert::TryInto;
    pub(crate) use std::marker::PhantomData;
    pub(crate) use std::fmt::Debug;
    pub(crate) use std::ops::{Deref, DerefMut};
    pub(crate) use either::Either;
    pub(crate) use either::Either::{Left, Right};
    use crate::tree::treenode::TreeNode;
    use crate::tree::traversal::Traversal;

    type Node<T> = Rc<RefCell<TreeNode<T>>>;

    pub(crate) mod treenode {
        use crate::tree::*;

        #[derive(Debug, PartialEq, Eq)]
        pub struct TreeNode<T> where T: Copy + Clone + Debug {
            pub value: T,
            pub left: Option<Node<T>>,
            pub right: Option<Node<T>>,
        }

        impl<T> TreeNode<T> where T: Copy + Clone + Debug {
            pub(crate) fn new(val: T) -> Self {
                TreeNode { value: val, left: None, right: None }
            }
        }
    }

    pub(crate) mod traversal {
        use crate::tree::*;

        pub enum TraversalDirection { Preorder, Inorder, Postorder, Levelorder, Spiralorder }
        pub trait Traversal<T> where T: Copy + Clone + Debug {
            fn traverse(&self, direction: TraversalDirection)
                        -> Either<Vec<Node<T>>, Vec<Vec<Node<T>>>> {
                match direction {
                    TraversalDirection::Preorder => Left(self.preorder()),
                    TraversalDirection::Inorder => Left(self.inorder()),
                    TraversalDirection::Postorder => Left(self.postorder()),
                    TraversalDirection::Levelorder => Right(self.levelorder()),
                    TraversalDirection::Spiralorder => Left(self.spiralorder()),
                }
            }

            fn traverse_values(&self, direction: TraversalDirection) -> Either<Vec<T>, Vec<Vec<T>>> {
                match self.traverse(direction) {
                    Left(nodes) => Left(nodes.into_iter().map(|n|
                        n.as_ref().borrow().value).collect()),
                    Right(levels) => Right(levels.into_iter().map(|l|
                        l.into_iter().map(|n| n.as_ref().borrow().value).collect()).collect()),
                }
            }

            fn preorder(&self) -> Vec<Node<T>>;
            fn inorder(&self) -> Vec<Node<T>>;
            fn postorder(&self) -> Vec<Node<T>>;
            fn levelorder(&self) -> Vec<Vec<Node<T>>>;
            fn spiralorder(&self) -> Vec<Node<T>>;
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Tree<T> where T: Copy + Clone + Debug {
        root: Option<Node<T>>,
    }

    impl<T> Tree<T> where T: Copy + Clone + Debug {
        pub(in crate::tree) fn new() -> Self {
            Self { root: None }
        }
    }

    impl<T> Traversal<T> for Tree<T> where T: Clone + Copy + Debug + PartialEq {
        fn preorder(&self) -> Vec<Node<T>> {
            let mut stack = Vec::new();
            let mut result = Vec::new();

            stack.push(self.root.clone());

            while let Some(node) = stack.pop() {
                if let Some(node) = node {
                    result.push(node.clone());
                    stack.push(node.as_ref().borrow().right.clone());
                    stack.push(node.as_ref().borrow().left.clone());
                }
            }
            result
        }

        fn inorder(&self) -> Vec<Node<T>> {
            let mut result = Vec::new();
            let mut stack = Vec::new();

            let mut node = self.root.clone();
            while !stack.is_empty() || node.is_some() {
                match node {
                    Some(n) => {
                        stack.push(Some(n.clone()));
                        node = n.as_ref().borrow().left.clone();
                    }
                    None => if let Some(Some(n)) = stack.pop() {
                        result.push(n.clone());
                        node = n.as_ref().borrow().right.clone();
                    },
                }
            }

            result
        }

        fn postorder(&self) -> Vec<Node<T>> {
            let mut stack = Vec::new();
            let mut result = Vec::new();
            let mut node = self.root.clone();

            loop {
                while node.is_some() {
                    stack.push(node.clone());
                    stack.push(node.clone());
                    node = node.unwrap().as_ref().borrow().left.clone();
                }

                match stack.pop() {
                    Some(n) => node = n,
                    None => return result,
                }

                let is_right_processed = || {
                    stack.last().is_some() && *stack.last().unwrap() == node
                };
                node = match is_right_processed() {
                    true => node.unwrap().as_ref().borrow().right.clone(),
                    false => {
                        result.push(node.unwrap().clone());
                        None
                    }
                }
            }
        }

        fn levelorder(&self) -> Vec<Vec<Node<T>>> {
            match self.root {
                None => Vec::new(),
                Some(_) => {
                    let mut level = Vec::new();
                    let mut levels = Vec::new();

                    level.push(self.root.clone().unwrap());
                    while !level.is_empty() {
                        levels.push(Vec::new());
                        let mut next_level = Vec::new();

                        while let Some(node) = level.pop() {
                            levels.last_mut().unwrap().insert(0, node.clone());

                            if let Some(node) = node.as_ref().borrow().left.clone() {
                                next_level.push(node);
                            }
                            if let Some(node) = node.as_ref().borrow().right.clone() {
                                next_level.push(node);
                            }
                        }
                        level = next_level;
                    }
                    levels
                }
            }
        }

        fn spiralorder(&self) -> Vec<Node<T>> {
            self.levelorder().concat()
        }
    }

    pub mod nodebuilder {
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
    }

    pub mod treebuilder {
        use crate::tree::*;
        use crate::tree::nodebuilder::NodeBuilder;

        pub struct TreeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
            tree: Tree<T>
        }

        impl<T> TreeBuilder<T> where T: Copy + Clone + Debug + PartialEq {
            pub fn new() -> Self { TreeBuilder { tree: Tree::new() } }

            pub fn with_root(mut self, node: Node<T>) -> Self {
                self.tree.root = Some(node);
                self
            }

            pub fn build(self) -> Tree<T> { self.tree }


            pub fn from_json(path: &str) -> Tree<T> { unimplemented!() }
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
    }

    use crate::tree::treebuilder::TreeBuilder;
}


fn main() {
    use crate::tree::traversal::Traversal;
    use crate::tree::traversal::TraversalDirection;
    use crate::tree::treebuilder::TreeBuilder;

    let tree = TreeBuilder::example_i32();
    println!("{:?}", tree.traverse_values(TraversalDirection::Preorder));
    println!("{:?}", tree.traverse_values(TraversalDirection::Inorder));
    println!("{:?}", tree.traverse_values(TraversalDirection::Postorder));
    println!("{:?}", tree.traverse_values(TraversalDirection::Levelorder));
    println!("{:?}", tree.traverse_values(TraversalDirection::Spiralorder));
}

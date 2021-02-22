use crate::tree::{Tree, Node};
use std::fmt::Debug;
use crate::tree::traversal::{TraversalDirection, Traversal};
use either::Either::Left;
use crate::tree::treebuilder::TreeBuilder;
use crate::tree::nodebuilder::NodeBuilder;
use crate::tree::includes::{TreeNode, RefCell, Rc};
use std::process::Output;
use std::ops::Add;
use std::borrow::BorrowMut;
use crate::utils::Sorted;

trait Similarity where Self: Sized {
    fn is_symmetric(&self) -> bool;
    fn is_univalued(&self) -> bool;
    fn is_leaf_similar(&self, other: &Self) -> bool;
}

impl<T> Similarity for Tree<T> where T: Clone + Copy + Debug + PartialEq {
    fn is_symmetric(&self) -> bool {
        fn is_mirror<T: Copy + Debug + PartialEq>(n1: &Option<Node<T>>, n2: &Option<Node<T>>) -> bool {
            match (n1, n2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    let (n1, n2) = (n1.borrow(), n2.borrow());
                    n1.value == n2.value
                        && is_mirror(&n1.left, &n2.right)
                        && is_mirror(&n1.right, &n2.left)
                }
                _ => false,
            }
        }
        is_mirror(&self.root, &self.root)
    }

    fn is_univalued(&self) -> bool {
        self.preorder().windows(2).all(
            |slice| match slice {
                [a, b] => a.borrow().value == b.borrow().value,
                _ => true
            }
        )
    }

    fn is_leaf_similar(&self, other: &Self) -> bool {
        self.find_leaf_values() == other.find_leaf_values()
    }
}

trait Arithmetics {
    fn sum_of_root_to_leaf_binary(&self) -> usize;
    fn average_level_values(&self) -> Vec<f64>;
    fn minimum_absolute_difference(&self) -> Vec<usize>;
    fn sum_in_range(&self, low: usize, high: usize) -> usize;
}

impl Arithmetics for Tree<i32> {
    fn sum_of_root_to_leaf_binary(&self) -> usize {
        unimplemented!()
    }

    fn average_level_values(&self) -> Vec<f64> {
        self.traverse_values(TraversalDirection::Levelorder).unwrap_right()
            .into_iter().map(|x| (1f64 / x.len() as f64) * x.into_iter()
            .fold(0f64, |acc, num| acc + num as f64) as f64).collect()
    }

    fn minimum_absolute_difference(&self) -> usize {
        self.traverse_values(TraversalDirection::Preorder)
            .unwrap_left().into_iter().map(|x| x.abs()).collect::<Vec<i32>>()
            .sorted()
            .windows(2).map(|slice| match slice {
            &[a, b] => b - a,
            _ => panic!("Should be impossible"),
        }).min().unwrap() as usize
    }

    fn sum_in_range(&self, low: usize, high: usize) -> usize {
        unimplemented!()
    }
}

trait Depth where Self: Sized {
    fn maximum_depth(&self) -> usize;
    fn minimum_depth(&self) -> usize;
}

impl<T> Depth for Tree<T> where T: Copy + Clone + Debug {
    fn maximum_depth(&self) -> usize {
        fn find_depth<T: Copy + Debug>(node: &Option<Node<T>>) -> usize {
            match node {
                Some(n) => 1 + usize::max(find_depth(&n.borrow().left), find_depth(&n.borrow().right)),
                None => 0,
            }
        }

        find_depth(&self.root)
    }
    fn minimum_depth(&self) -> usize {
        fn find_depth<T: Copy + Debug>(node: &Option<Node<T>>) -> usize {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    let depth = match (n.left.is_some(), n.right.is_some()) {
                        (true, false) => find_depth(&n.left),
                        (false, true) => find_depth(&n.right),
                        (true, true) => usize::min(
                            find_depth(&n.left),
                            find_depth(&n.right),
                        ),
                        (false, false) => 0,
                    };
                    depth + 1
                }
                None => 0,
            }
        }

        find_depth(&self.root)
    }
}

pub trait Combinatorics<T> where T: Copy + Clone + Debug {
    fn find_leaves(&self) -> Vec<Node<T>>;
    fn find_leaf_values(&self) -> Vec<T>;
}

impl<T> Combinatorics<T> for Tree<T> where T: Copy + Clone + Debug + PartialEq {
    fn find_leaves(&self) -> Vec<Node<T>> {
        self.inorder().into_iter()
            .filter(|x| x.borrow().left.is_none() && x.borrow().right.is_none())
            .collect()
    }

    fn find_leaf_values(&self) -> Vec<T> {
        self.find_leaves().into_iter().map(|x| x.borrow().value).collect()
    }
}

trait Operations where Self: Sized {
    fn merge(&self, other: &Self) -> Self;
}

impl<T> Operations for Tree<T> where T: Clone + Copy + Debug + Ord + Add<Output=T> {
    fn merge(&self, other: &Tree<T>) -> Self {
        fn merge_recursive<T>(n1: Option<Node<T>>, n2: Option<Node<T>>) -> Option<Node<T>> where T: Clone + Copy + Debug + Ord + Add<Output=T> {
            match (n1, n2) {
                (Some(n1), Some(n2)) => {
                    let (n1, n2) = (n1.borrow(), n2.borrow());

                    NodeBuilder::new(n1.value + n2.value).with_left(
                        merge_recursive(n1.left.clone(), n2.left.clone())
                    ).with_right(
                        merge_recursive(n1.right.clone(), n2.right.clone())
                    ).build()
                }
                (None, Some(n2)) => Some(n2),
                (Some(n1), None) => Some(n1),
                (None, None) => None,
            }
        }

        TreeBuilder::new().with_root(merge_recursive(self.root.clone(), other.root.clone())).build()
    }
}

enum ArrangementType {
    IncreasingOrder,
    Invert,
}

trait Arrangement where Self: Sized {
    fn arrange(&self, arrangement: ArrangementType) -> Self {
        match arrangement {
            ArrangementType::IncreasingOrder => self.increasing_order(),
            ArrangementType::Invert => self.invert(),
        }
    }

    fn increasing_order(&self) -> Self;
    fn invert(&self) -> Self;
}

impl<T> Arrangement for Tree<T> where T: Copy + Debug + Ord {
    fn increasing_order(&self) -> Self {
        let mut values = self.traverse_values(TraversalDirection::Inorder).unwrap_left();

        values.sort();
        let root = values.into_iter().rev().map(|x| NodeBuilder::leaf(x))
            .fold(None,
                  |right, parent| match parent {
                      Some(parent) => {
                          parent.as_ref().borrow_mut().right = right;
                          Some(parent)
                      }
                      None => None
                  },
            );
        TreeBuilder::new().with_root(root).build()
    }

    fn invert(&self) -> Self {
        fn invert_node<T>(node: Option<Node<T>>) -> Option<Node<T>> {
            match node {
                None => None,
                Some(n) => {
                    let (left, right) =
                        (invert_node(n.borrow().right.clone()),
                         invert_node(n.borrow().left.clone()));
                    let n = n.clone();
                    n.as_ref().borrow_mut().left = left;
                    n.as_ref().borrow_mut().right = right;
                    Some(n)
                }
            }
        }

        TreeBuilder::new().with_root(invert_node(self.root.clone())).build()
    }
}

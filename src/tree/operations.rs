use crate::tree::{Tree, Node};
use std::fmt::Debug;
use crate::tree::traversal::{TraversalDirection, Traversal};
use crate::tree::treebuilder::TreeBuilder;
use crate::tree::nodebuilder::NodeBuilder;
use std::ops::Add;
use crate::utils::Sorted;
use num::Num;
use crate::tree::includes::{TreeNode, RefCell, Rc};
use std::collections::HashMap;
use std::fs::read_to_string;

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
                    n1.val == n2.val
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
                [a, b] => a.borrow().val == b.borrow().val,
                _ => true
            }
        )
    }

    fn is_leaf_similar(&self, other: &Self) -> bool {
        self.find_leaf_values() == other.find_leaf_values()
    }
}

pub trait Arithmetics {
    fn find_least_common_ancestor_bst(&self, a: Option<Node<i32>>, b: Option<Node<i32>>) -> Option<Node<i32>>;
    fn find_smallest_path_by_values(&self) -> Vec<i32>;
    fn find_all_modes(&self) -> Vec<i32>;
    fn find_tilt(&self) -> i32;
    fn sum_of_root_to_leaf_binary(&self) -> i32;
    fn sum_in_range(&self, low: i32, high: i32) -> i32;
    fn two_sum(&self, target: i32) -> bool;
    fn average_level_values(&self) -> Vec<f64>;
    fn minimum_absolute_difference(&self) -> usize;
    fn is_valid_bst(&self) -> bool;
}

impl Arithmetics for Tree<i32> {
    fn find_least_common_ancestor_bst(&self, a: Option<Node<i32>>, b: Option<Node<i32>>) -> Option<Node<i32>> {
        match (a, b) {
            (Some(a), Some(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                let mut root = self.root.clone();
                while let Some(node) = root.clone() {
                    let node = node.borrow();
                    match i32::max(a.val, b.val) < node.val {
                        true => root = node.left.clone(),
                        false => match i32::min(a.val, b.val) > node.val {
                            true => root = node.right.clone(),
                            false => return root,
                        },
                    }
                }
                None
            }
            _ => None
        }
    }

    fn find_smallest_path_by_values(&self) -> Vec<i32> {
        self.find_paths_values().into_iter().map(|path|
            path.into_iter().rev().collect::<Vec<i32>>()).min().unwrap()
    }

    fn find_all_modes(&self) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        self.traverse_values(TraversalDirection::Preorder).unwrap_left()
            .into_iter().for_each(|e| *map.entry(e).or_default() += 1);

        match map.iter().max_by_key(|(_, v)| *v).map(|(_, count)| count.clone()) {
            Some(max_count) => map.into_iter()
                .filter(|&(_, count)| count == max_count).map(|(k, _)| k).collect(),
            None => Vec::new(),
        }
    }

    fn find_tilt(&self) -> i32 {
        fn recursive(node: &Option<Node<i32>>, tilt: &mut i32) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let left = recursive(&node.left, tilt);
                    let right = recursive(&node.right, tilt);

                    *tilt += i32::abs(left - right);
                    left + right + node.val
                }
                None => 0
            }
        }

        let mut tilt = 0;
        recursive(&self.root, &mut tilt);
        tilt
    }

    fn sum_of_root_to_leaf_binary(&self) -> i32 {
        self.find_paths_values().into_iter().map(|x| x.into_iter().sum::<i32>()).sum()
    }

    fn sum_in_range(&self, low: i32, high: i32) -> i32 {
        let mut stack = vec![self.root.clone()];
        let mut sum = 0;

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                if node.val >= low && node.val <= high { sum += node.val; }
                if node.val > low { stack.push(node.left.clone()) }
                if node.val < high { stack.push(node.right.clone()) }
            }
        }

        sum
    }

    fn two_sum(&self, target: i32) -> bool {
        let values = self.traverse_values(TraversalDirection::Preorder)
            .unwrap_left().sorted();
        values.iter().enumerate().clone().any(|(i, e1)|
            values.iter().skip(i + 1).enumerate().any(|(j, e2)| e1 + e2 == target))
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

    fn is_valid_bst(&self) -> bool {
        fn is_valid(node: &Option<Node<i32>>, left: i64, right: i64) -> bool {
            match node {
                Some(node) => {
                    let node = node.as_ref().borrow();

                    left < (node.val as i64) && (node.val as i64) < right
                        && is_valid(&node.left, left, node.val as i64)
                        && is_valid(&node.right, node.val as i64, right)
                }
                None => true
            }
        }
        is_valid(&self.root, i64::MIN, i64::MAX)
    }
}

trait Depth where Self: Sized {
    fn maximum_depth(&self) -> usize;
    fn minimum_depth(&self) -> usize;
    fn is_height_balanced(&self) -> bool;
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

    fn is_height_balanced(&self) -> bool {
        fn recursive<T: Copy + Clone + Debug>(node: &Option<Node<T>>) -> Option<i32> {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let left = recursive(&node.left);
                    let right = recursive(&node.right);
                    match (left, right) {
                        (Some(left), Some(right)) => match i32::abs(left - right) > 1 {
                            true => None,
                            false => Some(1 + i32::max(left, right)),
                        },
                        _ => None
                    }
                }
                None => Some(0),
            }
        }
        recursive(&self.root).is_some()
    }
}

pub trait Combinatorics<T> where T: Copy + Clone + Debug {
    fn find_leaves(&self) -> Vec<Node<T>>;
    fn find_leaf_values(&self) -> Vec<T>;
    fn find_paths(&self) -> Vec<Vec<Node<T>>>;
    fn find_paths_values(&self) -> Vec<Vec<T>>;
}

impl<T> Combinatorics<T> for Tree<T> where T: Copy + Clone + Debug + PartialEq {
    fn find_leaves(&self) -> Vec<Node<T>> {
        self.inorder().into_iter()
            .filter(|x| x.borrow().left.is_none() && x.borrow().right.is_none())
            .collect()
    }

    fn find_leaf_values(&self) -> Vec<T> {
        self.find_leaves().into_iter().map(|x| x.borrow().val).collect()
    }

    fn find_paths(&self) -> Vec<Vec<Node<T>>> {
        fn recursive<T: Copy + Clone + Debug>(node: Option<Node<T>>, mut path: Vec<Node<T>>, paths: &mut Vec<Vec<Node<T>>>) {
            if let Some(node) = node {
                path.push(node.clone());
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    paths.push(path.clone());
                }
                recursive(node.left.clone(), path.clone(), paths);
                recursive(node.right.clone(), path.clone(), paths);
            }
        }
        let mut paths = Vec::new();
        recursive(self.root.clone(), Vec::new(), &mut paths);
        paths
    }

    fn find_paths_values(&self) -> Vec<Vec<T>> {
        self.find_paths().into_iter().map(|path|
            path.into_iter().map(|n| n.as_ref().borrow().val).collect()).collect()
    }
}

pub trait Operations<T> where Self: Sized, T: Clone + Copy + Debug + PartialEq {
    fn merge(&self, other: &Self) -> Self;
    fn find_value(&self, value: T) -> Vec<Node<T>>;
    fn is_subtree(&self, other: &Self) -> bool;
}

impl<T> Operations<T> for Tree<T> where T: Clone + Copy + Debug + Ord + Add<Output=T> {
    fn merge(&self, other: &Tree<T>) -> Self {
        fn merge_recursive<T>(n1: Option<Node<T>>, n2: Option<Node<T>>) -> Option<Node<T>> where T: Clone + Copy + Debug + Ord + Add<Output=T> {
            match (n1, n2) {
                (Some(n1), Some(n2)) => {
                    let (n1, n2) = (n1.borrow(), n2.borrow());

                    NodeBuilder::new(n1.val + n2.val).with_left(
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

    fn find_value(&self, value: T) -> Vec<Node<T>> {
        self.preorder().into_iter().filter(|n| n.borrow().val == value).collect()
    }

    fn is_subtree(&self, other: &Self) -> bool {
        match (self.root.clone(), other.root.clone()) {
            (None, None) => true,
            (Some(_), Some(subtree)) => self.preorder().into_iter().any(|x| x == subtree),
            (None, Some(_)) => false,
            (Some(_), None) => true,
        }
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
        fn invert_node<T: Copy + Debug>(node: Option<Node<T>>) -> Option<Node<T>> {
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

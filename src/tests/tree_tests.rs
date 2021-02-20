use crate::tree::treebuilder::TreeBuilder;
use either::*;
use crate::tree::Tree;
use crate::tree::traversal::{Traversal, TraversalDirection};

struct ExampleTree {
    tree: Tree<i32>
}
impl ExampleTree {
    fn new() -> Self {
        ExampleTree { tree: TreeBuilder::example_i32() }
    }
}
#[test]
fn example_preorder() {
    assert_eq!(ExampleTree::new().tree.traverse_values(TraversalDirection::Preorder),
               Left(Vec::<i32>::from([5,3,1,4,8])), "Should return Left([5,3,1,4,8])");
}
#[test]
fn example_inorder() {
    assert_eq!(ExampleTree::new().tree.traverse_values(TraversalDirection::Inorder),
               Left(Vec::<i32>::from([1,3,4,5,8])), "Should return Left([1,3,4,5,8])");
}

#[test]
fn example_postorder() {
    assert_eq!(ExampleTree::new().tree.traverse_values(TraversalDirection::Postorder),
               Left(Vec::<i32>::from([1,4,3,8,5])), "Should return Left([1,4,3,8,5])");
}
#[test]
fn example_levelorder() {
    assert_eq!(ExampleTree::new().tree.traverse_values(TraversalDirection::Levelorder),
               Right(Vec::<Vec<i32>>::from([vec![5],vec![3,8],vec![1,4]])), "Should return Right([[5],[3,8],[1,4]])");
}

#[test]
fn example_spiralorder() {
    assert_eq!(ExampleTree::new().tree.traverse_values(TraversalDirection::Spiralorder),
               Left(Vec::<i32>::from([5,3,8,1,4])), "Should return Left([5,8,3,1,4])");
}

use crate::tree::traversal::{TraversalDirection, Traversal};
use crate::tree::treebuilder::TreeBuilder;

pub mod tree;
pub mod utils;

#[cfg(test)]
mod tests;

fn main() {
    let a = TreeBuilder::example_i32();

    println!("{:?}", a.traverse_values(TraversalDirection::Preorder));
    println!("{:?}", a.traverse_values(TraversalDirection::Inorder));
    println!("{:?}", a.traverse_values(TraversalDirection::Postorder));
    println!("{:?}", a.traverse_values(TraversalDirection::Levelorder));
    println!("{:?}", a.traverse_values(TraversalDirection::Spiralorder));
}

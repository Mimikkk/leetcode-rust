use crate::tree::traversal::{TraversalDirection, Traversal};
use crate::tree::treebuilder::TreeBuilder;
use crate::tree::operations::Combinatorics;
use crate::tree::operations::Arithmetics;
use rand::thread_rng;
use std::collections::HashMap;
pub mod tree;
pub mod utils;

#[cfg(test)]
mod tests;

fn to_num(digits: Vec<u8>) -> u64 {
    let mut i = 0;
    digits.into_iter().rev().fold(0, |acc, num| {
        let num = acc + num as u64 * u64::pow(10, i);
        i += 1;
        num
    })
}

fn main() {
    let tree = tree::treebuilder::TreeBuilder::example_i32();
    println!("{}", tree.find_tilt());
}

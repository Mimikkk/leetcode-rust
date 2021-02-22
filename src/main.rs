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
    let mut map: HashMap<i32, usize> = HashMap::new();
    for e in vec![1, 1, 1, 2, 3, 4, 4, 4] {
        *map.entry(e).or_default() += 1;
    }
    let max_count = map.iter().max_by_key(|(_, v)| *v).map(|(_, count)| count.clone()).unwrap();
    let vec: Vec<i32> = map.into_iter().filter(|&(_, count)| count == max_count).map(|(k, _)| k).collect();
    println!("{:?}", vec);
}

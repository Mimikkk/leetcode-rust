#![allow(dead_code)]
use crate::algorithms::is_isomorphic;
pub mod tree;
pub mod utils;
pub mod algorithms;


#[cfg(test)]
mod tests;


fn main() {
    println!("{}", is_isomorphic(String::from("abcdef"), String::from("fedcba")));
}

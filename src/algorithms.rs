use std::fs::read_to_string;
use std::ops::Index;
use std::collections::HashMap;
pub fn pascal_triangle_i(num_rows: usize) -> Vec<Vec<i32>> {
    match num_rows {
        0 => vec![],
        1 => vec![vec![1]],
        2 => vec![vec![1], vec![1, 1]],
        _ => {
            let mut result = pascal_triangle_i(2);
            for _ in 3..=num_rows {
                let vec: Vec<i32> = result.last().unwrap().windows(2).map(|slice| match slice {
                    &[a, b] => a + b,
                    _ => panic!("Is impossible"),
                }).collect();
                result.push([vec![1], vec, vec![1]].concat());
            }
            result
        }
    }
}

pub fn pascal_triangle_ii(row_index: usize) -> Vec<i32> {
    match row_index {
        0 => vec![1],
        1 => vec![1, 1],
        _ => {
            let mut result = pascal_triangle_ii(1);
            for _ in 2..=row_index {
                result = [vec![1], result.windows(2).map(|slice| match slice {
                    &[a, b] => a + b,
                    _ => panic!("Is impossible"),
                }).collect(), vec![1]].concat();
            }
            result
        }
    }
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    prices.windows(2).for_each(|slice| match slice {
        &[pre_p, p] => if p > pre_p { profit += p - pre_p }
        _ => panic!("Impossible")
    });
    profit
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i1 in 0..numbers.len() {
        if let Ok(i2) = numbers.binary_search(&(target - numbers[i1])) {
            return vec![i1 as i32 + 1, i2 as i32 + 1];
        }
    }

    vec![]
}

pub fn convert_title_to_number(s: String) -> i32 {
    s.chars().fold(0, |acc, c| acc * 26 + c as i32 - 64)
}

pub fn convert_number_to_title(mut n: i32) -> String {
    let mut result = String::new();
    while n > 0 {
        result.insert(0, (65 + ((n - 1) % 26)) as u8 as char);
        n = (n - 1) / 26;
    }
    result
}

pub fn num_sub(s: String) -> i32 {
    fn natural_sum(n: i64) -> i64 {
        (n * (n + 1)) / 2
    }
    s.chars().chain("0".chars()).fold((0, 0), |(acc, count), e|
        match e {
            '0' => ((acc + natural_sum(count)) % 1000000007, 0),
            '1' => (acc, count + 1),
            _ => panic!("Invalid input")
        }).0 as i32
}

pub fn count_homogenous(s: String) -> i32 {
    fn natural_sum(n: i64) -> i64 {
        (n * (n + 1)) / 2
    }

    s.chars().chain("\0".chars()).fold((0, 0, None), |(acc, count, prev), e|
        match (Some(e) == prev, prev) {
            (_, None) => (0, 1, Some(e)),
            (true, Some(_)) => (acc, count + 1, Some(e)),
            (false, Some(_)) => (acc + natural_sum(count) % 1000000007, 1, Some(e)),
        }).0 as i32
}

pub fn max_power(s: String) -> i32 {
    s.chars().chain("\0".chars()).fold((0, 0, None), |(max, count, prev), e|
        match (Some(e) == prev, prev) {
            (_, None) => (1, 1, Some(e)),
            (true, _) => (i32::max(max, count), count + 1, prev),
            (false, _) => (i32::max(max, count), 1, Some(e)),
        }).0
}

pub fn trailing_zeroes_of_factorial(n: u32) -> u32 {
    match n / 5 {
        0 => 0,
        _ => n / 5 + trailing_zeroes_of_factorial(n / 5)
    }
}

pub fn reverse_bits(n: u32) -> u32 {
    (0..32).fold((0, n), |(m, n), _| (m << 1 | n & 1, n >> 1)).0
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let [mut d1, mut d2] = [[0u16; 256]; 2];
    !s.chars().zip(t.chars()).enumerate()
        .map(|(i, (c1, c2))| (i as u16, (c1 as usize, c2 as usize)))
        .any(|(i, (c1, c2))| {
            let bool_ = d1[c1] != d2[c2];
            d1[c1] = i + 1;
            d2[c2] = i + 1;
            bool_
        })
}

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut map: HashMap<&str, char> = HashMap::new();
    let words: Vec<&str> = s.split(' ').collect();
    match pattern.len() == words.len() {
        false => false,
        true => words.into_iter().zip(pattern.chars()).find(|&(word, c)|
            match map.get_mut(&word) {
                Some(val) => *val != c,
                None => if map.values().any(|x| *x == c) {
                    true
                } else {
                    map.insert(word, c);
                    false
                }
            }).is_none(),
    }
}

struct Primes {}

impl Primes {
    pub fn new() -> Self { Self {} }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::*;
    #[test]
    fn test_title_to_number() {
        assert_eq!(convert_title_to_number(String::from("A")), 1, "Should be 1");
        assert_eq!(convert_title_to_number(String::from("D")), 4, "Should be 4");
        assert_eq!(convert_title_to_number(String::from("AB")), 28, "Should be 28");
        assert_eq!(convert_title_to_number(String::from("ZY")), 701, "Should be 701");
    }
    #[test]
    fn test_convert_number_to_title() {
        assert_eq!(convert_number_to_title(1), String::from("A"), "Should be A");
        assert_eq!(convert_number_to_title(4), String::from("D"), "Should be D");
        assert_eq!(convert_number_to_title(28), String::from("AB"), "Should be AB");
        assert_eq!(convert_number_to_title(701), String::from("ZY"), "Should be ZY");
    }
    #[test]
    fn test_num_sub() {
        assert_eq!(num_sub(String::from("0110111")), 9, "Should be 9");
        assert_eq!(num_sub(String::from("101")), 2, "Should be 2");
        assert_eq!(num_sub(String::from("111111")), 21, "Should be 21");
        assert_eq!(num_sub(String::from("1".repeat(100_000))), 49965, "Should be 49965 otherwise would overflow");
    }
    #[test]
    fn test_count_homogenous() {
        assert_eq!(count_homogenous(String::from("abbcccaa")), 13, "3 + 1 + 2 + 1 + 3 + 2 + 1 = 13");
        assert_eq!(count_homogenous(String::from("xy")), 2, "1+1=2");
        assert_eq!(count_homogenous(String::from("zzzzz")), 15, "1+2+3+4+5=15");
    }
    #[test]
    fn test_max_power() {
        assert_eq!(max_power(String::from("abbcccddddeeeeedcba")), 5, "The substring \"eeeee\" is of length 5 with the character 'e' only.");
        assert_eq!(max_power(String::from("abbccc")), 3, "The substring \"ccc\"");
    }
    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 964176192u32, "Should be 964176192");
    }
    #[test]
    fn test_is_isomorphic() {
        assert_eq!(is_isomorphic(String::from("aah"), String::from("bbg")), true, "Should be true");
        assert_eq!(is_isomorphic(String::from("bbbaaaba"), String::from("aaabbbba")), false, "Should be false");
    }
    #[test]
    fn test_word_pattern() {
        assert_eq!(word_pattern(String::from("abba"), String::from("cat dog dog cat")), true);
        assert_eq!(word_pattern(String::from("abba"), String::from("dog cat cat dog")), true);
    }
}
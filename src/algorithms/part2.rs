use std::collections::HashMap;

// region [[Skończone]]
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    for (i, num) in sorted_nums.into_iter().enumerate() {
        if !map.contains_key(&num) { map.insert(num, i); }
    }

    nums.into_iter().map(|x| *map.entry(x).or_default() as i32).collect()
}

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let s = s.into_bytes();
    let mut result = vec![0; s.len()];
    for (i, e) in indices.into_iter().zip(s.into_iter()) {
        result[i as usize] = e;
    }
    String::from_utf8(result).unwrap_or_default()
}

pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    let mut prod = 1;
    while n > 0 {
        sum += n % 10;
        prod *= n % 10;
        n /= 10;
    }
    prod - sum
}

pub fn number_of_steps(mut num: i32) -> i32 {
    (31 + std::cmp::max(1, num.count_ones()) - num.leading_zeros()) as i32
}

pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    nums.chunks(2).map(|x| match x {
        &[a, b] => (a, b),
        _ => panic!("impossible"),
    }).map(|(f, v)| vec![v; f as usize]).flatten().collect()
}
//endregion

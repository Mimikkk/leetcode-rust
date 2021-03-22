use std::collections::HashMap;
use std::cmp::Ordering;
use std::iter;
use crate::utils::Sorted;

// region [[Done]]
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

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
    nums.sort_by_key(|&(i, _)| i);

    match (0..nums.len()).find_map(|i| (i + 1..nums.len()).find_map(|j|
        match nums[i].1 + nums[j].1 == target {
            true => Some(vec![nums[i].0 as i32, nums[j].0 as i32]),
            false => None
        })) {
        Some(val) => val,
        None => vec![-1, -1],
    }
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

pub fn add_digits(mut n: i32) -> i32 {
    loop {
        if n < 10 { break n; }
        let mut sum = 0;

        while n > 0 {
            sum += n % 10;
            n /= 10;
        };
        n = sum;
    }
}

pub fn is_ugly(n: i32) -> bool {
    match n > 0 {
        true => {
            (2..=5).fold(n, |mut acc, i| loop {
                if acc % i != 0 { break acc; };
                acc /= i;
            }) == 1
        }
        false => false,
    }
}

pub fn count_primes(n: i32) -> i32 {
    match n > 1 {
        true => {
            let mut prime = vec![true; n as usize];
            if let Some(b) = prime.get_mut(0) {
                *b = false;
            }
            if let Some(b) = prime.get_mut(1) {
                *b = false;
            }

            for i in 0..((n as f32).sqrt() + 1f32) as usize {
                if let Some(&b) = prime.get(i) {
                    if b {
                        for j in (i * i..n as usize).step_by(i) {
                            prime[j] = false
                        }
                    }
                }
            }

            prime.into_iter().filter(|x| *x).count() as i32
        }
        false => 0
    }
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    fn format_ranges(ranges: Vec<(i32, i32)>) -> Vec<String> {
        ranges.into_iter().map(|(a, b)| match a == b {
            false => format!("{}->{}", a, b),
            true => format!("{}", a)
        }).collect()
    }
    if nums.is_empty() { return vec![]; }

    let mut ranges = vec![(nums[0], nums[0])];
    for num in nums.into_iter().skip(1) {
        if let Some((_, last)) = ranges.last_mut() {
            if num - *last == 1 { *last += 1 } else { ranges.push((num, num)) }
        }
    }
    format_ranges(ranges)
}

pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort();
    nums2.sort();
    let mut result: Vec<i32> =
        nums1.into_iter().filter(|x| nums2.binary_search(x).is_ok()).collect();
    result.dedup();
    result
}

pub fn intersect2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort();
    nums2.sort();
    let mut result = vec![];
    while let (Some(&n1), Some(&n2)) = (nums1.last(), nums2.last()) {
        match n1.cmp(&n2) {
            Ordering::Less => nums2.pop(),
            Ordering::Equal => {
                result.push(n1);
                nums1.pop();
                nums2.pop()
            }
            Ordering::Greater => nums1.pop(),
        };
    }
    result
}

pub fn is_perfect_square(x: i32) -> bool {
    fn bs(left: i64, right: i64, target: i64) -> bool {
        if left > right { return false; }

        let mid = (left + right) / 2;
        match (mid * mid).cmp(&target) {
            Ordering::Less => bs(mid + 1, right, target),
            Ordering::Equal => true,
            Ordering::Greater => bs(left, mid - 1, target),
        }
    }
    bs(1, x as i64, x as i64)
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let (mut a, mut b) = (vec![0; 255], vec![0; 255]);
    for c in ransom_note.into_bytes() { a[c as usize] += 1; }
    for c in magazine.into_bytes() { b[c as usize] += 1; }

    a.into_iter().enumerate().all(|(i, n)| b[i] >= n)
}

pub fn find_the_difference(s: String, t: String) -> char {
    let (s, l) = if s.len() > t.len() { (t, s) } else { (s, t) };
    for (c1, c2) in l.chars().zip(s.chars().chain(iter::repeat('\0'))) {
        match (c1, c2) {
            (_, _) => {}
        }
    };
    'c'
}

pub fn read_binary_watch(n: i32) -> Vec<String> {
    let n = n as u32;
    (0..1024u16).filter(|x| x.count_ones() == n).filter(|x| (x & 0b1111) < 12)
        .map(|i| format!("{}:{:0>2}", i & 0b1111, i >> 4)).collect::<Vec<_>>().sorted()
}

pub fn my_pow(mut x: f64, n: i32) -> f64 {
    if n == 0 { return 1f64; }
    let mut n = n as i64;

    if n < 0 {
        x = 1f64 / x;
        n = n.abs();
    }

    let mut y = 1f64;
    while n > 1 {
        if n % 2 == 1 {
            y *= x;
            x *= x;
            n = (n - 1) / 2;
        } else {
            x *= x;
            n /= 2;
        }
    }
    x * y
}

pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
    fn powmod(a: i32, b: i32) -> i32 {
        (0..b).fold((1, a % 1337), |(result, a), _| ((result * a) % 1337, a)).0
    }

    match b.pop() {
        Some(last_digit) => powmod(super_pow(a, b), 10) * powmod(a, last_digit) % 1337,
        None => 1,
    }
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let (mut i, mut reach, n) = (0, 0, nums.len());
    while i < n && i <= reach {
        reach = usize::max(i + nums[i] as usize, reach);
        i += 1;
    }
    i == n
}

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    intervals.sort();

    for (head, tail) in intervals.into_iter().map(|row| (row[0], row[1])) {
        match result.last_mut() {
            Some(last) =>
                if last[1] >= head {
                    last[1] = last[1].max(tail)
                } else { result.push(vec![head, tail]) },
            None => result.push(vec![head, tail]),
        }
    }
    result
}

pub fn unique_paths(n: i32, m: i32) -> i32 {
    fn n_choose_k(n: i64, k: i64) -> i64 {
        (1..=k).fold(1, |acc, i| acc * (n - k + i) / i)
    }
    n_choose_k((m + n - 2) as i64, (n - 1) as i64) as i32
}
/*
    1     1     1     1     1     1     1     1     1
    1     2     3     4     5     6     7     8     9
    1     3     6    10    15    21    28    36    45
    1     4    10    20    35    56    84   120   165
    1     5    15    35    70   126   210   330   495
    1     6    21    56   126   252   462   792  1287
    1     7    28    84   210   462   924  1716  3003
    1     8    36   120   330   792  1716  3432  6435
    1     9    45   165   495  1287  3003  6435 12870
*/
pub fn unique_paths_dp(n: i32, m: i32) -> i32 {
    let (n, m) = (n as usize, m as usize);
    let mut dp = vec![vec![1; m]; n];
    (1..n).for_each(|i| (1..m).for_each(|j| dp[i][j] = dp[i - 1][j] + dp[i][j - 1]));
    dp[n - 1][m - 1]
}

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 };
    (1..m).for_each(|i| dp[0][i] = if obstacle_grid[0][i] == 1 { 0 } else { dp[0][i - 1] });
    (1..n).for_each(|i| dp[i][0] = if obstacle_grid[i][0] == 1 { 0 } else { dp[i - 1][0] });
    (1..n).for_each(|i| (1..m).for_each(|j|
        dp[i][j] = if obstacle_grid[i][j] == 1 { 0 } else { dp[i - 1][j] + dp[i][j - 1] }
    ));
    dp[n - 1][m - 1]
}

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (n, m) = (matrix.len(), matrix[0].len());
    let mut row_indices = vec![];
    let mut col_indices = vec![];
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 0 {
                if !row_indices.contains(&i) { row_indices.push(i) }
                if !col_indices.contains(&j) { col_indices.push(j) }
            }
        }
    }
    for r in row_indices { for i in 0..m { matrix[r][i] = 0; } }
    for c in col_indices { for i in 0..n { matrix[i][c] = 0; } }
}
// endregion

pub mod gol {
    struct LifeGame {
        current_generation: Vec<Vec<bool>>,
        n: usize,
        m: usize,
    }
    trait AsBoolean {
        fn as_bool(&self) -> Vec<Vec<bool>>;
    }
    trait AsI32 {
        fn as_i32(&self) -> Vec<Vec<i32>>;
    }

    impl AsBoolean for Vec<Vec<i32>> {
        fn as_bool(&self) -> Vec<Vec<bool>> {
            let (n, m) = (self.len(), self[0].len());
            let mut sol = vec![vec![false; m]; n];
            (0..n).for_each(|i| (0..m).for_each(|j| sol[i][j] = self[i][j] != 0));
            sol
        }
    }
    impl AsI32 for Vec<Vec<bool>> {
        fn as_i32(&self) -> Vec<Vec<i32>> {
            let (n, m) = (self.len(), self[0].len());
            let mut sol = vec![vec![0; m]; n];
            (0..n).for_each(|i| (0..m).for_each(|j| sol[i][j] = self[i][j] as i32));
            sol
        }
    }
    impl AsI32 for LifeGame {
        fn as_i32(&self) -> Vec<Vec<i32>> {
            self.current_generation.as_i32()
        }
    }

    impl LifeGame {
        fn new(board: &mut Vec<Vec<i32>>) -> Self {
            match board.is_empty() {
                true => Self { current_generation: vec![], n: 0, m: 0 },
                false => Self { current_generation: board.as_bool(), n: board.len(), m: board[0].len() },
            }
        }


        const NEIGHS: [(i32, i32); 8] =
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        fn count_neighbours(&self, i: usize, j: usize) -> usize {
            let mut neighbour_count = 0;
            for &(x, y) in Self::NEIGHS.iter()
                .filter(|(a, b)| self.is_valid_cell(i as i32 + a, j as i32 + b)) {
                neighbour_count += self.current_generation[x as usize][y as usize] as usize;
            }
            neighbour_count
        }
        fn is_valid_cell(&self, i: i32, j: i32) -> bool {
            i > 0 && i < self.n as i32 && j > 0 && j < self.m as i32
        }
        fn is_alive(&self, i: usize, j: usize) -> bool {
            match self.count_neighbours(i, j) {
                2..=3 => true,
                _ => false,
            }
        }

        fn next_generation(&self) -> Vec<Vec<bool>> {
            let mut next_gen = self.current_generation.clone();

            for i in 0..self.n {
                for j in 0..self.m {
                    next_gen[i][j] = self.is_alive(i, j);
                }
            }

            next_gen
        }

        fn solve(&mut self) {
            loop {
                let next_gen = self.next_generation();
                if self.current_generation == next_gen { break; }
                self.current_generation = next_gen;
            }
        }
        fn solved(mut self) -> Self {
            self.solve();
            self
        }
    }


    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        *board = LifeGame::new(board).solved().as_i32();
    }
}


#[cfg(test)]
mod tests {
    use crate::algorithms::*;
    use crate::utils::Sorted;
    // region [[Done]]
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
    #[test]
    fn test_add_numbers() {
        assert_eq!(add_digits(38), 2, "Should be 2");
    }
    #[test]
    fn test_if_ugly() {
        assert_eq!(is_ugly(240), true);
        assert_eq!(is_ugly(1), true);
        assert_eq!(is_ugly(128), true);
        assert_eq!(is_ugly(7), false);
        assert_eq!(is_ugly(u32::MAX as i32), false);
    }
    #[test]
    fn test_intersection() {
        assert_eq!(intersection(vec![1, 2, 3, 4, 5], vec![5, 3, 7]), vec![3, 5])
    }
    #[test]
    fn test_read_binary_watch() {
        assert_eq!(read_binary_watch(1),
                   ["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
                       .iter().map(|&x| String::from(x)).collect::<Vec<_>>().sorted());
        assert_eq!(read_binary_watch(2),
                   [
                       "0:03", "0:05", "0:06", "0:09", "0:10", "0:12", "0:17", "0:18", "0:20",
                       "0:24", "0:33", "0:34", "0:36", "0:40", "0:48", "1:01", "1:02", "1:04",
                       "1:08", "1:16", "1:32", "2:01", "2:02", "2:04", "2:08", "2:16", "2:32",
                       "3:00", "4:01", "4:02", "4:04", "4:08", "4:16", "4:32", "5:00", "6:00",
                       "8:01", "8:02", "8:04", "8:08", "8:16", "8:32", "9:00", "10:00"
                   ].iter().map(|&x| String::from(x)).collect::<Vec<_>>().sorted()
        );
    }
    // endregion
    #[test]
    fn test_uni_paths_dp() {
        for i in 1..10 {
            for j in 1..10 {
                assert_eq!(unique_paths_dp(i, j), unique_paths(i, j));
            }
        }
    }
}

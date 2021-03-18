use std::collections::{HashMap, HashSet};


pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
    nums.sort_by_key(|&(i, _)| i);

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i].1 + nums[j].1 == target {
                return vec![nums[i].0 as i32, nums[j].0 as i32];
            }
        }
    }
    panic!("No Valid Solution");
}
pub fn two_sum_onepass(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let complement = target - nums[i];
        if map.contains_key(&complement) {
            return vec![map[&complement], i as i32];
        }
        map.insert(nums[i], i as i32);
    }
    panic!("No Valid Solution");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

type Node = Box<ListNode>;
pub fn add_two_numbers(mut l1: Option<Node>, mut l2: Option<Node>) -> Option<Node> {
    let (mut val, mut carry) = (0, false);

    let mut head = None;
    let mut cur: Option<&mut Node> = None;
    return loop {
        val = carry as i32;
        if let Some(n) = l1 {
            val += n.val;
            l1 = n.next;
        }
        if let Some(n) = l2 {
            val += n.val;
            l2 = n.next;
        }
        carry = val > 9;
        val %= 10;

        match cur.is_some() {
            true => {
                cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
                cur = cur.unwrap().next.as_mut();
            }
            false => {
                head = Some(Box::new(ListNode::new(val)));
                cur = head.as_mut();
            }
        };

        if l1.is_none() && l2.is_none() && !carry { break head; }
    };
}

fn length_of_longest_substring_firsttime(s: String) -> i32 {
    let (mut cur, mut longest, mut set) = (0, 0, HashSet::new());
    for i in 0..s.len() {
        for e in s.chars().skip(i) {
            if set.contains(&e) {
                break;
            }
            set.insert(e);
            cur += 1;
        }
        longest = cur.max(longest);
        cur = 0;
        set.clear();
    }
    cur.max(longest)
}
pub fn length_of_longest_substring_secondtime(s: String) -> i32 {
    let (mut best, mut start, mut map) = (0, 0, HashMap::new());
    for (i, c) in s.chars().enumerate().map(|(i, c)| (i as i32, c)) {
        if map.contains_key(&c) {
            start = start.max(*map.entry(c).or_default() + 1);
        }
        *map.entry(c).or_default() = i;
        best = best.max(i - start + 1);
    }
    best
}
pub fn length_of_longest_substring_thirdtime(s: String) -> i32 {
    let (mut best, mut start, mut map) = (0, 0, HashMap::new());
    for (i, c) in s.chars().enumerate().map(|(i, c)| (i as i32, c)) {
        if let Some(previous_index) = map.insert(c, i) {
            start = start.max(previous_index + 1);
        }
        best = best.max(i - start + 1);
    }
    best
}

fn roman_to_int(s: String) -> i32 {
    let map: HashMap<char, i32> = [
        ('I', 1), ('V', 5), ('X', 10), ('L', 50),
        ('C', 100), ('D', 500), ('M', 1000)
    ].iter().cloned().collect();

    let (mut result, mut previous, mut current) = (0, 0, 0);
    for c in s.chars().rev() {
        current = map[&c];
        result += if current >= previous { current } else { -current };
        previous = current;
    }
    result
    // Obligatoryjny jednolinijkowiec:
    // s.chars().rev().fold((0, 0), |(acc, prev), c| {
    //     (acc + if map[&c] >= prev { map[&c] } else { -map[&c] }, map[&c])
    // }).0
}
fn roman_to_int_bigbrain(s: String) -> i32 {
    s.chars().rfold(0, |accum, c| {
        accum + match c {
            'I' if accum >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if accum >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if accum >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("String isn't a roman numeral"),
        }
    })
}

fn reverse(mut n: i32) -> i32 {
    let mut result: i32 = 0;
    return loop {
        result = match result.checked_mul(10) {
            Some(v) => match v.checked_add(n % 10) {
                Some(v) => v,
                None => return 0
            },
            None => return 0
        };
        n /= 10;
        if n == 0 { break result; }
    };
}

#[cfg(test)]
mod tests {
    use crate::ghost::firstbatch::*;
    #[test]
    fn test_reverse() {
        assert_eq!(reverse(7134), 4317);
        assert_eq!(reverse(-7134), -4317);
        assert_eq!(reverse(-7134), -4317);
        assert_eq!(reverse(i32::MIN), 0);
        assert_eq!(reverse(i32::MAX), 0);
    }
    #[test]
    fn test_length_of_longest_substring_firsttime() {
        assert_eq!(length_of_longest_substring_firsttime(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring_firsttime(String::from("aabc")), 3);
        assert_eq!(length_of_longest_substring_firsttime(String::from("aab")), 2);
        assert_eq!(length_of_longest_substring_firsttime(String::from("aabvd")), 4);
        assert_eq!(length_of_longest_substring_firsttime(String::from("dvdf")), 3);
    }
    #[test]
    fn test_length_of_longest_substring_secondtime() {
        assert_eq!(length_of_longest_substring_secondtime(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring_secondtime(String::from("aabc")), 3);
        assert_eq!(length_of_longest_substring_secondtime(String::from("aab")), 2);
        assert_eq!(length_of_longest_substring_secondtime(String::from("aabvd")), 4);
        assert_eq!(length_of_longest_substring_secondtime(String::from("dvdf")), 3);
    }
    #[test]
    fn test_length_of_longest_substring_thirdtime() {
        assert_eq!(length_of_longest_substring_thirdtime(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring_thirdtime(String::from("aabc")), 3);
        assert_eq!(length_of_longest_substring_thirdtime(String::from("aab")), 2);
        assert_eq!(length_of_longest_substring_thirdtime(String::from("aabvd")), 4);
        assert_eq!(length_of_longest_substring_thirdtime(String::from("dvdf")), 3);
    }
    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(Some(Box::new(ListNode::new(2))),
                                   Some(Box::new(ListNode::new(4)))),
                   Some(Box::new(ListNode::new(6))));
    }
}
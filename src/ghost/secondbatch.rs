use std::str::Chars;
use crate::algorithms::can_construct;
use std::iter;
pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 { return x; }
    let mut low: i64 = 1;
    let mut high: i64 = x as i64;
    let mut mid: i64 = 0;
    let mut ans: i32 = 0;

    while low <= high {
        mid = (low + high) / 2;
        let square = mid * mid;

        if square == x as i64 {
            return mid as i32;
        } else if square > x as i64 {
            high = mid - 1;
        } else {
            low = mid + 1;
            ans = mid as i32;
        }
    }
    ans
}
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            _ => if Some(c) != stack.pop() { return false; }
        }
    }
    stack.is_empty()
}
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i1 in 0..numbers.len() {
        if let Ok(i2) = numbers.binary_search(&(target - numbers[i1])) {
            return vec![i1 as i32 + 1, i2 as i32 + 1];
        }
    }

    vec![]
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len() - 1);
    let mut mid;
    while low <= high {
        mid = (high - low) / 2 + low;
        if nums[mid] < target {
            low = mid + 1;
        } else {
            if nums[mid] == target && nums[mid - 1] != target {
                return mid as i32;
            } else {
                high = mid - 1;
            }
        }
    }
    low as i32
}
pub fn str_str(s: String, subs: String) -> i32 {
    if subs.is_empty() { return 0; }
    let s = s.into_bytes();
    let subs = subs.into_bytes();

    for (i, c) in s.clone().into_iter().enumerate() {
        if subs.iter().zip(s.iter().skip(i).chain(iter::repeat(&0)))
            .all(|(c1, c2)| c1 == c2) { return i as i32; }
    }
    -1
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let mut newsize = 1;
    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            nums[newsize] = nums[i];
            newsize += 1;
        };
    }
    nums.resize(newsize, 0);
    newsize as i32
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
pub fn merge_two_lists(mut l1: Option<Node>, mut l2: Option<Node>) -> Option<Node> {
    let mut head = None;
    let mut cur: Option<&mut Node> = None;
    return loop {
        match (l1.clone(), l2.clone()) {
            (Some(n1), Some(n2)) => {}
            (Some(n), None) => {
                match cur.is_some() {
                    true => {
                        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(n.val)));
                        cur = cur.unwrap().next.as_mut();
                    }
                    false => {
                        head = Some(Box::new(ListNode::new(n.val)));
                        cur = head.as_mut();
                    }
                }

                l1 = n.next;
            }
            (None, Some(n)) => {
                match cur.is_some() {
                    true => {
                        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(n.val)));
                        cur = cur.unwrap().next.as_mut();
                    }
                    false => {
                        head = Some(Box::new(ListNode::new(n.val)));
                        cur = head.as_mut();
                    }
                }

                l2 = n.next;
            }
            (None, None) => break head,
        }
    };
}
pub fn longest_common_prefix(vec: Vec<String>) -> String {
    if vec.is_empty() { return String::new(); }

    let mut result = String::new();

    let mut iters: Vec<_> = vec.iter().map(|x| x.chars()).collect();
    while let Some(current) = iters[0].next() {
        if !(1..iters.len()).all(|i| match iters[i].next() {
            Some(other) => current == other,
            None => false,
        }) { return result; }
        result.push(current);
    };
    result
}
pub fn count_primes(n: i32) -> i32 {
    if n < 1 { return 0; }
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
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| acc ^ n)
}

#[cfg(test)]
mod tests {
    use crate::ghost::secondbatch::*;
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(Vec::from([String::from("aab"), String::from("aaaba"), String::from("aabs")])), String::from("aa"));
        assert_eq!(longest_common_prefix(Vec::from([])), String::from(""));
        assert_eq!(longest_common_prefix(Vec::from([String::from("")])), String::from(""));
        assert_eq!(longest_common_prefix(Vec::from([String::from(""), String::from("")])), String::from(""));
        assert_eq!(longest_common_prefix(Vec::from([String::from(""), String::from("a")])), String::from(""));
        assert_eq!(longest_common_prefix(Vec::from([String::from("a"), String::from("a")])), String::from("a"));
        assert_eq!(longest_common_prefix(Vec::from([String::from("a")])), String::from("a"));
    }
    #[test]
    fn test_remove_duplicates() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2, 3, 3, 3, 3]), 3);
    }
}

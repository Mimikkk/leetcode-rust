struct NumArray { range_sum: Vec<i32> }

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        nums.reverse();

        let mut range_sum = vec![];
        let mut sum_so_far = None;
        while let Some(num) = nums.pop() {
            sum_so_far = match sum_so_far {
                None => Some(num),
                Some(sum) => Some(sum + num),
            };
            range_sum.push(sum_so_far.unwrap());
        }

        Self { range_sum }
    }

    fn sum_range(&self, i: i32, j: i32) -> Option<i32> {
        match (self.range_sum.get((i - 1) as usize), self.range_sum.get(j as usize)) {
            (Some(&a), Some(&b)) => Some(b - a),
            (None, Some(&b)) => Some(b),
            (Some(&a), None) => match self.range_sum.last() {
                Some(&b) => Some(b - a),
                None => None,
            },
            (None, None) => None,
        }
    }
}


#[test]
fn test_numarray_new() {
    let array = NumArray::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(array.range_sum, vec![1, 3, 6, 10, 15]);
}

#[test]
fn test_numarray_range() {
    let array = NumArray::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(array.sum_range(1, 2), Some(5));
}
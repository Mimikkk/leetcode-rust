pub struct PrimeIterator {
    pub primes_so_far: Vec<u64>,
    checked_so_far: u64,
}

impl PrimeIterator {
    fn new() -> Self {
        Self { primes_so_far: vec![], checked_so_far: 1 }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.checked_so_far += 1;
            if self.primes_so_far.iter().all(|&x| self.checked_so_far % x != 0) {
                self.primes_so_far.push(self.checked_so_far);
                break Some(self.checked_so_far);
            }
        }
    }
}

pub fn find_primes_before_given(n: u64) -> Vec<u64> {
    let mut iterator = PrimeIterator::new();
    let mut primes = Vec::new();
    loop {
        let p = iterator.next().unwrap_or(u64::MAX);
        if p > n { break primes; }
        primes.push(p);
    }
}

pub fn find_prime_factors(mut n: u64) -> Vec<u64> {
    let mut iterator = PrimeIterator::new();
    let mut prime_factors = vec![];
    while n != 1 {
        let p = iterator.next().unwrap_or(u64::MAX);
        while n % p == 0 {
            prime_factors.push(p);
            n /= p;
        }
    }
    prime_factors
}

pub fn is_prime(n: u64) -> bool {
    let mut iterator = PrimeIterator::new();
    loop {
        let p = iterator.next().unwrap_or(u64::MAX);
        if p == n { break true; }
        if p > n { break false; }
    }
}

#[test]
fn test_prime_iterator() {
    let mut primes = PrimeIterator::new();
    assert_eq!(primes.next(), Some(2));
    assert_eq!(primes.next(), Some(3));
    assert_eq!(primes.next(), Some(5));
    assert_eq!(primes.next(), Some(7));
    assert_eq!(primes.next(), Some(11));
    assert_eq!(primes.next(), Some(13));
    assert_eq!(primes.next(), Some(17));
    assert_eq!(primes.next(), Some(19));
    assert_eq!(primes.next(), Some(23));
}
#[test]
fn test_is_prime() {
    assert_eq!(is_prime(17), true);
    assert_eq!(is_prime(14), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(21), false);
    assert_eq!(is_prime(23), true);
}
#[test]
fn test_find_prime_factors() {
    assert_eq!(find_prime_factors(240), vec![2, 2, 2, 2, 3, 5])
}
fn test_find_primes_before_given() {
    assert_eq!(find_primes_before_given(12), vec![2, 3, 5, 7, 11])
}
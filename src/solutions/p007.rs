use crate::solutions::Solution;

pub struct Problem007;

impl Problem007 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem007 {
    fn number(&self) -> u32 { 7 }

    fn title(&self) -> &'static str {
        "10001st prime"
    }

    fn solve(&self) -> String {
        let mut primes = vec![2];
        let mut n = 3;
        while primes.len() < 10_001 {
            if primes.iter().all(|&p| n % p != 0) {
                primes.push(n);
            }
            n += 2;
        }
        primes.last().unwrap().to_string()
    }
}
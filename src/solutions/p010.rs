use crate::solutions::Solution;

pub struct Problem010;

impl Problem010 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem010 {
    fn number(&self) -> u32 { 10 }

    fn title(&self) -> &'static str {
        "Summation of primes"
    }

    fn solve(&self) -> String {
        let mut sieve = vec![true; 2_000_000];
        sieve[0] = false;
        sieve[1] = false;
        let mut sum = 0;
        for i in 2..sieve.len() {
            if sieve[i] {
                sum += i;
                for j in (i..sieve.len()).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
        sum.to_string()
    }
}
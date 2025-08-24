use crate::solutions::Solution;

pub struct Problem037;

impl Problem037 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem037 {
    fn number(&self) -> u32 { 37 }

    fn title(&self) -> &'static str {
        "Truncatable Primes"
    }

    fn solve(&self) -> String {

        let is_prime = |n: usize| {
            if n < 2 {
                return false;
            }
            for i in 2..=((n as f64).sqrt() as usize) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        };

        let is_truncatable_prime = |n: usize| {
            let s = n.to_string();
            (1..=s.len()).all(|i| {
                let left = &s[..i];
                is_prime(left.parse::<usize>().unwrap())
            }) && (1..=s.len()-1).rev().all(|i| {
                let right = &s[i..]                    ;
                is_prime(right.parse::<usize>().unwrap())
            })
        };

        let mut total = 0;

        // infinite loop until we find 11 truncatable primes
        let mut count = 0;
        let mut n = 11;
        while count < 11 {
            if is_truncatable_prime(n) {
                total += n;
                count += 1;
            }
            n += 1;
        }

        total.to_string()
    }

}
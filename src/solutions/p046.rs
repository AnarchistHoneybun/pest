use crate::solutions::Solution;

pub struct Problem046;

impl Problem046 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem046 {
    fn number(&self) -> u32 { 46 }

    fn title(&self) -> &'static str {
        "Goldbach's Other Conjecture"
    }

    fn solve(&self) -> String {
        fn is_prime(n: u64) -> bool {
            if n < 2 { return false; }
            if n == 2 { return true; }
            if n % 2 == 0 { return false; }
            for i in (3..=((n as f64).sqrt() as u64)).step_by(2) {
                if n % i == 0 { return false; }
            }
            true
        }

        fn is_composite(n: u64) -> bool {
            if n < 4 { return false; }
            for i in 2..=((n as f64).sqrt() as u64) {
                if n % i == 0 { return true; }
            }
            false
        }

        fn can_be_written_as_prime_plus_twice_square(n: u64) -> bool {
            let mut k = 1;
            while 2 * k * k < n {
                let remainder = n - 2 * k * k;
                if is_prime(remainder) {
                    return true;
                }
                k += 1;
            }
            false
        }

        let mut n = 9;
        loop {
            if n % 2 == 1 && is_composite(n) {
                if !can_be_written_as_prime_plus_twice_square(n) {
                    return n.to_string();
                }
            }
            n += 2;
        }
    }
}
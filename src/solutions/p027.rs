use crate::solutions::Solution;

pub struct Problem027;

impl Problem027 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem027 {
    fn number(&self) -> u32 { 27 }

    fn title(&self) -> &'static str {
        "Quadratic primes"
    }

    fn solve(&self) -> String {
        let mut max = 0;
        let mut product = 0;
        for a in -999..1000 {
            for b in -1000..=1000 {
                let mut n = 0;
                while is_prime(n * n + a * n + b) {
                    n += 1;
                }
                if n > max {
                    max = n;
                    product = a * b;
                }
            }
        }
        product.to_string()
    }
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
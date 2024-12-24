use crate::solutions::Solution;

pub struct Problem012;

impl Problem012 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem012 {
    fn number(&self) -> u32 { 12 }

    fn title(&self) -> &'static str {
        "Highly divisible triangular number"
    }

    fn solve(&self) -> String {
        let mut n = 1;
        let mut triangle = 0;
        loop {
            triangle += n;
            n += 1;
            let mut divisors = 0;
            for i in 1..=(triangle as f64).sqrt() as u64 {
                if triangle % i == 0 {
                    divisors += 2;
                }
            }
            if divisors > 500 {
                break;
            }
        }
        triangle.to_string()
    }
}
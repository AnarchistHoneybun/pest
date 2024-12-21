use crate::solutions::Solution;

pub struct Problem002;

impl Problem002 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem002 {
    fn number(&self) -> u32 { 2 }

    fn title(&self) -> &'static str {
        "Even Fibonacci numbers"
    }

    fn solve(&self) -> String {
        let mut sum = 0;
        let mut a = 1;
        let mut b = 2;

        while b < 4_000_000 {
            if b % 2 == 0 {
                sum += b;
            }
            let c = a + b;
            a = b;
            b = c;
        }
        sum.to_string()
    }
}
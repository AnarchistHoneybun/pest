use crate::solutions::Solution;

pub struct Problem021;

impl Problem021 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem021 {
    fn number(&self) -> u32 { 21 }

    fn title(&self) -> &'static str {
        "Amicable Numbers"
    }

    fn solve(&self) -> String {
        let mut sum = 0;
        for i in 2..10_000 {
            let sum_divisors = (1..i).filter(|&j| i % j == 0).sum::<u32>();
            let sum_divisors2 = (1..sum_divisors).filter(|&j| sum_divisors % j == 0).sum::<u32>();
            if i == sum_divisors2 && i != sum_divisors {
                sum += i;
            }
        }
        sum.to_string()
    }
}
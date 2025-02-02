use crate::solutions::Solution;

pub struct Problem030;

impl Problem030 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem030 {
    fn number(&self) -> u32 { 30 }

    fn title(&self) -> &'static str {
        "Digit Fifth Powers"
    }

    fn solve(&self) -> String {
        let mut sum = 0;
        for i in 2..1000000 {
            let num = i.to_string();
            let cust_i = num.chars().map(|c| c.to_digit(10).unwrap().pow(5)).sum::<u32>();
            if cust_i == i { sum += i; }
        }
        sum.to_string()
    }
}
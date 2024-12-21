use crate::solutions::Solution;

pub struct Problem001;

impl Problem001 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem001 {
    fn number(&self) -> u32 { 1 }

    fn title(&self) -> &'static str {
        "Multiples of 3 or 5"
    }

    fn solve(&self) -> String {
        (1..1000)
            .filter(|&x| x % 3 == 0 || x % 5 == 0)
            .sum::<u64>()
            .to_string()
    }
}
use crate::solutions::Solution;

pub struct Problem015;

impl Problem015 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem015 {
    fn number(&self) -> u32 { 15 }

    fn title(&self) -> &'static str {
        "Lattice Paths"
    }

    fn solve(&self) -> String {
        let answer: u64 = (0..20).fold(1, |acc, i| acc * (40 - i) / (i + 1));
        answer.to_string()
    }
}
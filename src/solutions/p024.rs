use crate::solutions::Solution;
use itertools::Itertools;

pub struct Problem024;

impl Problem024 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem024 {
    fn number(&self) -> u32 { 24 }

    fn title(&self) -> &'static str {
        "Lexicographic Permutations"
    }

    fn solve(&self) -> String {
        let answer = (0..10).permutations(10).nth(999_999).unwrap().iter().join("");
        answer
    }
}
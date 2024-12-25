use crate::solutions::Solution;

pub struct Problem022;

impl Problem022 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem022 {
    fn number(&self) -> u32 { 22 }

    fn title(&self) -> &'static str {
        "Names Scores"
    }

    fn solve(&self) -> String {
        let mut names = include_str!("../../resources/p022_names.txt")
            .split(',')
            .map(|name| name.trim_matches('"'))
            .collect::<Vec<&str>>();
        names.sort();
        let answer: u64 = names.iter().enumerate().map(|(i, name)| {
            let score: u64 = name.chars().map(|c| c as u64 - 64).sum();
            score * (i as u64 + 1)
        }).sum();
        answer.to_string()
    }
}
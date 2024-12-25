use crate::solutions::Solution;

pub struct Problem023;

impl Problem023 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem023 {
    fn number(&self) -> u32 { 23 }

    fn title(&self) -> &'static str {
        "Non-abundant Sums"
    }

    fn solve(&self) -> String {
        const LIMIT: usize = 28124;
        let mut sums = vec![0; LIMIT];

        for i in 1..LIMIT {
            for j in (i * 2..LIMIT).step_by(i) { sums[j] += i; }
        }

        let abundant: Vec<_> = (0..LIMIT).filter(|&i| sums[i] > i).collect();
        let mut express = vec![false; LIMIT];

        for &i in &abundant {
            for &j in &abundant {
                if i + j >= LIMIT { break; }
                express[i + j] = true;
            }
        }

        (0..LIMIT).filter(|&i| !express[i]).sum::<usize>().to_string()
    }
}
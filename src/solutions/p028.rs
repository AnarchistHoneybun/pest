use crate::solutions::Solution;

pub struct Problem028;

impl Problem028 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem028 {
    fn number(&self) -> u32 { 28 }

    fn title(&self) -> &'static str {
        "Number Spiral Diagonals"
    }

    fn solve(&self) -> String {
        let size = 1001;
        let mut ans = 1;

        for i in (3..=size).step_by(2) {
            ans += 4*i*i - 6*(i - 1);
        }

        ans.to_string()
    }
}
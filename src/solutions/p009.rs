use crate::solutions::Solution;

pub struct Problem009;

impl Problem009 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem009 {
    fn number(&self) -> u32 { 9 }

    fn title(&self) -> &'static str {
        "Special Pythagorean Triplet"
    }

    fn solve(&self) -> String {
        for a in 1..1000 {
            for b in a..1000 {
                let c = 1000 - a - b;
                if a * a + b * b == c * c {
                    return (a * b * c).to_string();
                }
            }
        }
        "No solution found".to_string()
    }
}
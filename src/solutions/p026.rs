use crate::solutions::Solution;

pub struct Problem026;

impl Problem026 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem026 {
    fn number(&self) -> u32 { 26 }

    fn title(&self) -> &'static str {
        "Reciprocal Cycles"
    }

    fn solve(&self) -> String {
        let mut max = 0;
        let mut d = 0;
        for i in 2..1000 {
            let mut remainders = vec![0; i];
            let mut value = 1;
            let mut position = 0;
            while remainders[value] == 0 && value != 0 {
                remainders[value] = position;
                value = value * 10 % i;
                position += 1;
            }
            if position - remainders[value] > max {
                max = position - remainders[value];
                d = i;
            }
        }
        d.to_string()
    }
}
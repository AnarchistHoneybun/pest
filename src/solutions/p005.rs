use crate::solutions::Solution;

pub struct Problem005;

impl Problem005 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem005 {
    fn number(&self) -> u32 { 5 }

    fn title(&self) -> &'static str {
        "Smallest multiple"
    }

    fn solve(&self) -> String {
        let mut n:usize = 1;
        for i in 1..21 {
            n = n * i / gcd(n, i);
        }
        n.to_string()
    }
}

fn gcd(p0: usize, p1: usize) -> usize {
    if p1 == 0 {
        p0
    } else {
        gcd(p1, p0 % p1)
    }
}
use crate::solutions::Solution;

pub struct Problem003;

impl Problem003 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem003 {
    fn number(&self) -> u32 { 3 }

    fn title(&self) -> &'static str {
        "Largest prime factor"
    }

    fn solve(&self) -> String {
        let mut n: u64 = 600_851_475_143;
        let mut i = 2;

        while i * i <= n {
            if n % i == 0 {
                n /= i;
            } else {
                i += 1;
            }
        }
        n.to_string()
    }
}
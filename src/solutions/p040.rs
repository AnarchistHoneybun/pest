use crate::solutions::Solution;

pub struct Problem040;

impl Problem040 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem040 {
    fn number(&self) -> u32 {
        40
    }

    fn title(&self) -> &'static str {
        "Champernowne's Constant"
    }

    fn solve(&self) -> String {
        let target = 1_000_000;
        let mut champernowne = String::new();
        let mut n = 1;
        while champernowne.len() < target {
            champernowne.push_str(&n.to_string());
            n += 1;
        }
        let result = (1..=6)
            .map(|i| champernowne.chars().nth(10u32.pow(i - 1) as usize - 1).unwrap().to_digit(10).unwrap())
            .product::<u32>();
        result.to_string()
    }
}

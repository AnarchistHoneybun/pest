use crate::solutions::Solution;

pub struct Problem038;

impl Problem038 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem038 {
    fn number(&self) -> u32 { 38 }

    fn title(&self) -> &'static str {
        "Pandigital Multiples"
    }

    fn solve(&self) -> String {
        let mut result = 0;
        for n in 1..=9999 {
            let mut s = String::new();
            let mut k = 1;
            while s.len() < 9 {
                s.push_str(&(n * k).to_string());
                k += 1;
            }
            if s.len() == 9 && k > 2 && s.chars().all(|c| c != '0') && s.chars().collect::<std::collections::HashSet<_>>().len() == 9 {
                result = result.max(s.parse::<usize>().unwrap());
            }
        }
        result.to_string()
    }

}
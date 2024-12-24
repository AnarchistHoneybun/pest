use std::collections::HashMap;
use crate::solutions::Solution;

pub struct Problem014;

impl Problem014 {
    pub fn new() -> Self {
        Self
    }

    fn collatz_chain_length(&self, x: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if x == 1 {
            return 1;
        }
        if let Some(&length) = cache.get(&x) {
            return length;
        }
        let next = if x % 2 == 0 { x / 2 } else { x * 3 + 1 };
        let length = self.collatz_chain_length(next, cache) + 1;
        cache.insert(x, length);
        length
    }
}

impl Solution for Problem014 {
    fn number(&self) -> u32 { 14 }

    fn title(&self) -> &'static str {
        "Largest collatz sequence"
    }

    fn solve(&self) -> String {
        let mut cache = HashMap::new();
        let ans = (1..1_000_000)
            .max_by_key(|&x| self.collatz_chain_length(x, &mut cache))
            .unwrap();
        ans.to_string()
    }
}
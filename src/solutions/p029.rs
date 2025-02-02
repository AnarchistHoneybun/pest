use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Problem029;

impl Problem029 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem029 {
    fn number(&self) -> u32 { 29 }

    fn title(&self) -> &'static str {
        "Distinct Powers"
    }

    fn solve(&self) -> String {
        let mut values = HashSet::new();
        for a in 2u8..=100 {
            for b in 2..=100 {
                let term = (a as f64).powf(b as f64);
                let bits: u64 = unsafe { std::mem::transmute(term) };
                values.insert(bits);
            }
        }
        values.len().to_string()
    }
}
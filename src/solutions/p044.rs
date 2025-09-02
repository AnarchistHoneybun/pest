use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Problem044;

impl Problem044 {
    pub fn new() -> Self {
        Self
    }
    
    fn pentagonal(n: u64) -> u64 {
        n * (3 * n - 1) / 2
    }
}

impl Solution for Problem044 {
    fn number(&self) -> u32 { 44 }

    fn title(&self) -> &'static str {
        "Pentagon Numbers"
    }

    fn solve(&self) -> String {
        let max_n = 3000;
        let mut pentagonal_set = HashSet::new();
        let mut pentagonals = Vec::new();
        
        for n in 1..=max_n {
            let p = Self::pentagonal(n);
            pentagonal_set.insert(p);
            pentagonals.push(p);
        }
        
        let mut min_difference = u64::MAX;
        
        // Check all pairs (i, j) where i < j
        for i in 0..pentagonals.len() {
            for j in (i + 1)..pentagonals.len() {
                let pj = pentagonals[i];
                let pk = pentagonals[j];
                
                let sum = pj + pk;
                let diff = pk - pj;
                
                if pentagonal_set.contains(&sum) && pentagonal_set.contains(&diff) {
                    if diff < min_difference {
                        min_difference = diff;
                    }
                }
                
                if diff > min_difference {
                    break;
                }
            }
        }
        
        min_difference.to_string()
    }
}
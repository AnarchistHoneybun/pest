use std::collections::HashSet;
use itertools::Itertools;
use crate::solutions::Solution;

pub struct Problem032;

impl Problem032 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem032 {
    fn number(&self) -> u32 { 32 }

    fn title(&self) -> &'static str {
        "Pandigital Products"
    }

    fn solve(&self) -> String {
        let digits: Vec<u32> = (1..=9).collect();
        let mut products = HashSet::new();
        
        for perm in digits.iter().permutations(9) {
            
            
            for i in 1..8 {
                for j in 1..9-i {
                    if i+j>=9{
                        continue;
                    }
                    
                    let a = dig_to_num(&perm[0..i]);
                    let b = dig_to_num(&perm[i..i+j]);
                    let c = dig_to_num(&perm[(i+j)..]);
                    
                    if a*b == c {
                        products.insert(c);
                    }
                    
                }
            }
        }
        
        products.into_iter().sum::<u32>().to_string()
    }
}

fn dig_to_num(digits: &[&u32]) -> u32 {
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}
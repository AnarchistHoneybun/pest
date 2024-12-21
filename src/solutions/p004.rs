use crate::solutions::Solution;

pub struct Problem004;

impl Problem004 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem004 {
    fn number(&self) -> u32 { 4 }

    fn title(&self) -> &'static str {
        "Largest palindrome product"
    }

    fn solve(&self) -> String {
        let mut max = 0;
        for i in 100..1000 {
            for j in 100..1000 {
                let product = i * j;
                if product > max && product.to_string() == product.to_string().chars().rev().collect::<String>() {
                    max = product;
                }
            }
        }
        max.to_string()
    }
}
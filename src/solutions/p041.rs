use crate::solutions::Solution;
use itertools::Itertools;

pub struct Problem041;

impl Problem041 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem041 {
    fn number(&self) -> u32 {
        41
    }

    fn title(&self) -> &'static str {
        "Pandigital Prime"
    }

    fn solve(&self) -> String {
        let is_prime = |n: usize| {
            if n < 2 {
                return false;
            }
            for i in 2..=((n as f64).sqrt() as usize) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        };

        let digits = vec!['7', '6', '5', '4', '3', '2', '1'];
        for perm in digits.iter().permutations(7) {
            let num_str: String = perm.iter().map(|&&c| c).collect();
            let num = num_str.parse::<usize>().unwrap();
            if is_prime(num) {
                return num.to_string();
            }
        }

        let digits = vec!['4', '3', '2', '1'];
        for perm in digits.iter().permutations(4) {
            let num_str: String = perm.iter().map(|&&c| c).collect();
            let num = num_str.parse::<usize>().unwrap();
            if is_prime(num) {
                return num.to_string();
            }
        }

        "0".to_string()
    }
}

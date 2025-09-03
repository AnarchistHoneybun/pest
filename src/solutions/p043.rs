use crate::solutions::Solution;
use itertools::Itertools;

pub struct Problem043;

impl Problem043 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem043 {
    fn number(&self) -> u32 {
        43
    }

    fn title(&self) -> &'static str {
        "Sub-string Divisibility"
    }

    fn solve(&self) -> String {
        let digits = "0123456789";
        let mut pandigitals = Vec::new();

        for perm in digits.chars().permutations(10) {
            let num_str: String = perm.iter().collect();
            if !num_str.starts_with('0') {
                pandigitals.push(num_str);
            }
        }

        let is_substring_divisible = |s: &str| {
            let primes = [2, 3, 5, 7, 11, 13, 17];
            for i in 1..=7 {
                let sub_num = s[i..i + 3].parse::<usize>().unwrap();
                if sub_num % primes[i - 1] != 0 {
                    return false;
                }
            }
            true
        };

        let mut sum: usize = 0;

        for pandigital in pandigitals {
            if is_substring_divisible(&pandigital) {
                sum += pandigital.parse::<usize>().unwrap();
            }
        }

        sum.to_string()
    }
}

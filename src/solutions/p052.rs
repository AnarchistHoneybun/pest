use crate::solutions::Solution;

pub struct Problem052;

impl Problem052 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem052 {
    fn number(&self) -> u32 {
        52
    }

    fn title(&self) -> &'static str {
        "Permuted Multiples"
    }

    fn solve(&self) -> String {

        fn is_permutation(a: u64, b: u64) -> bool {
            let mut a_digits = a.to_string().chars().collect::<Vec<_>>();
            let mut b_digits = b.to_string().chars().collect::<Vec<_>>();
            a_digits.sort();
            b_digits.sort();
            a_digits == b_digits
        }

        for i in 1..=100_000_000 {
            if is_permutation(i, i * 2) {
            let mut all_permuted = true;
            for j in 3..=6 {
                if !is_permutation(i, i * j) {
                all_permuted = false;
                break;
                }
            }
            if all_permuted {
                return i.to_string();
            }
            }
        }
        "No solution found".to_string()
    }
}

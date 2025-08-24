use crate::solutions::Solution;

pub struct Problem036;

impl Problem036 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem036 {
    fn number(&self) -> u32 { 36 }

    fn title(&self) -> &'static str {
        "Double-base Palindromes"
    }

    fn solve(&self) -> String {
        let is_palindrome = |n: usize| {
            let s = n.to_string();
            s == s.chars().rev().collect::<String>()
        };

        let mut total = 0;
        for i in 1..=1_000_000 {
            if is_palindrome(i) && is_palindrome(format!("{:b}", i).parse::<usize>().unwrap()) {
                total += i;
            }
        }

        total.to_string()
    }

}
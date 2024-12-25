use crate::solutions::Solution;

pub struct Problem020;

impl Problem020 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem020 {
    fn number(&self) -> u32 { 20 }

    fn title(&self) -> &'static str {
        "Factorial digit sum"
    }

    fn solve(&self) -> String {
        let mut digits = vec![1];
        for i in 1..=100 {
            let mut carry = 0;
            for d in digits.iter_mut() {
                let val = *d * i + carry;
                *d = val % 10;
                carry = val / 10;
            }
            while carry > 0 {
                digits.push(carry % 10);
                carry /= 10;
            }
        }
        digits.iter().sum::<u32>().to_string()
    }
}
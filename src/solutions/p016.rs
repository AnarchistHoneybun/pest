use crate::solutions::Solution;

pub struct Problem016;

impl Problem016 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem016 {
    fn number(&self) -> u32 { 16 }

    fn title(&self) -> &'static str {
        "Power Digit Sum"
    }

    fn solve(&self) -> String {
        let mut digits = vec![1];
        for _ in 0..1_000 {
            let mut carry = 0;
            for d in digits.iter_mut() {
                let val = *d * 2 + carry;
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
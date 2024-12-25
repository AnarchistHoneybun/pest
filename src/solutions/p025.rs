use crate::solutions::Solution;

pub struct Problem025;

impl Problem025 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem025 {
    fn number(&self) -> u32 { 25 }

    fn title(&self) -> &'static str {
        "1000-digit Fibonacci Number"
    }

    fn solve(&self) -> String {
        let mut a = vec![0];
        let mut b = vec![1];
        let mut n = 1;
        while b.len() < 1000 {
            let tmp = b.clone();
            let mut carry = 0;
            for i in 0..b.len() {
                if i >= a.len() {
                    a.push(0);
                }
                let mut digit = a[i];
                digit = b[i] + digit + carry;
                carry = digit / 10;
                a[i] = digit % 10;
            }
            if carry > 0 {
                a.push(carry);
            }
            b = a;
            a = tmp;
            n += 1;
        }
        n.to_string()
    }
}
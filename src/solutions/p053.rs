use crate::solutions::Solution;

pub struct Problem053;

impl Problem053 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem053 {
    fn number(&self) -> u32 { 53 }

    fn title(&self) -> &'static str {
        "Combinatoric Selections"
    }

    fn solve(&self) -> String {
        fn binomial_coefficient(n: u32, r: u32) -> u64 {
            if r > n { return 0; }
            if r == 0 || r == n { return 1; }
            let r = r.min(n - r);
            let mut result = 1u64;
            for i in 0..r {
                if result > u64::MAX / (n - i) as u64 {
                    return u64::MAX;
                }
                result = result * (n - i) as u64 / (i + 1) as u64;
                if result > 1_000_000 {
                    return result;
                }
            }
            result
        }
        
        let mut count = 0;
        for n in 1..=100 {
            let mut r_min = None;
            for r in 0..=n/2 {
                if binomial_coefficient(n, r) > 1_000_000 {
                    r_min = Some(r);
                    break;
                }
            }
            if let Some(r_min) = r_min {
                let r_max = n - r_min;
                count += r_max - r_min + 1;
            }
        }
        
        count.to_string()
    }
}
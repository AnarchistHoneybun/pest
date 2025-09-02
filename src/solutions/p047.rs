use crate::solutions::Solution;

pub struct Problem047;

impl Problem047 {
    pub fn new() -> Self {
        Self
    }

    fn count_distinct_prime_factors(mut n: u64) -> u32 {
        let mut count = 0;
        
        if n % 2 == 0 {
            count += 1;
            while n % 2 == 0 {
                n /= 2;
            }
        }
        
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                count += 1;
                while n % i == 0 {
                    n /= i;
                }
            }
            i += 2;
        }
        
        if n > 1 {
            count += 1;
        }
        
        count
    }
}

impl Solution for Problem047 {
    fn number(&self) -> u32 { 47 }

    fn title(&self) -> &'static str {
        "Distinct Prime Factors"
    }

    fn solve(&self) -> String {
        let target_factors = 4;
        let consecutive_count = 4;
        
        let mut n = 2;
        
        loop {
            let mut all_match = true;
            
            for i in 0..consecutive_count {
                if Self::count_distinct_prime_factors(n + i) != target_factors {
                    all_match = false;
                    break;
                }
            }
            
            if all_match {
                return n.to_string();
            }
            
            n += 1;
        }
    }
}
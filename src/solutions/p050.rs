use crate::solutions::Solution;
use crate::utils::prime_sieve;
use std::collections::HashSet;

pub struct Problem050;

impl Problem050 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem050 {
    fn number(&self) -> u32 {
        50
    }

    fn title(&self) -> &'static str {
        "Consecutive Prime Sum"
    }

    fn solve(&self) -> String {
        const LIMIT: u64 = 1_000_000;
        
        let primes = prime_sieve(2, LIMIT);
        let prime_set: HashSet<u64> = primes.iter().cloned().collect();
        
        let mut max_length = 0;
        let mut result_prime = 0;
        
        // Try all possible starting positions
        for start in 0..primes.len() {
            let mut sum = 0;
            let mut length = 0;
            
            // Add consecutive primes starting from 'start'
            for end in start..primes.len() {
                sum += primes[end];
                length += 1;
                
                // If sum exceeds limit, break
                if sum >= LIMIT {
                    break;
                }
                
                // If sum is prime and length is greater than current max
                if prime_set.contains(&sum) && length > max_length {
                    max_length = length;
                    result_prime = sum;
                }
            }
        }
        
        result_prime.to_string()
    }
}

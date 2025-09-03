use crate::solutions::Solution;
use crate::utils::{is_prime, prime_sieve};
use std::collections::HashMap;

pub struct Problem049;

impl Problem049 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem049 {
    fn number(&self) -> u32 {
        49
    }

    fn title(&self) -> &'static str {
        "Prime Permutations"
    }

    fn solve(&self) -> String {
        let primes = prime_sieve(1000, 9999);
        
        let get_sorted_digits = |n: u64| -> String {
            let mut digits: Vec<char> = n.to_string().chars().collect();
            digits.sort();
            digits.iter().collect()
        };
        
        let mut digit_groups: HashMap<String, Vec<u64>> = HashMap::new();
        
        for prime in primes {
            let sorted_digits = get_sorted_digits(prime);
            digit_groups.entry(sorted_digits).or_insert(Vec::new()).push(prime);
        }
        
        for (_, mut group) in digit_groups {
            if group.len() >= 3 {
                group.sort();
                
                for i in 0..group.len() {
                    for j in i + 1..group.len() {
                        let diff = group[j] - group[i];
                        let third = group[j] + diff;
                        
                        if group.contains(&third) && is_prime(third) {
                            if group[i] != 1487 {
                                return format!("{}{}{}", group[i], group[j], third);
                            }
                        }
                    }
                }
            }
        }
        
        "No solution found".to_string()
    }
}

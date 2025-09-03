use crate::solutions::Solution;
use std::collections::{HashMap, HashSet};

pub struct Problem051;

impl Problem051 {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PatternKey {
    pattern: String,
    positions: Vec<usize>,
}

impl PatternKey {
    fn new(number: u32, replacement_positions: &[usize]) -> Self {
        let number_str = number.to_string();
        let mut pattern: Vec<char> = number_str.chars().collect();
        
        for &pos in replacement_positions {
            if pos < pattern.len() {
                pattern[pos] = '*';
            }
        }
        
        let mut sorted_positions = replacement_positions.to_vec();
        sorted_positions.sort_unstable();
        
        PatternKey {
            pattern: pattern.iter().collect(),
            positions: sorted_positions,
        }
    }
}

#[derive(Debug, Clone)]
struct FamilyInfo {
    prime_count: usize,
    smallest_prime: Option<u32>,
}

struct PrimeFamilyFinder {
    prime_set: HashSet<u32>,
    primes_list: Vec<u32>,
    pattern_cache: HashMap<PatternKey, FamilyInfo>,
    cache_hits: u32,
    cache_misses: u32,
}

impl PrimeFamilyFinder {
    fn new(max_number: u32) -> Self {
        let prime_set = Self::sieve_of_eratosthenes(max_number);
        let mut primes_list: Vec<u32> = prime_set.iter().copied().collect();
        primes_list.sort_unstable();
        
        Self {
            prime_set,
            primes_list,
            pattern_cache: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }
    
    fn sieve_of_eratosthenes(n: u32) -> HashSet<u32> {
        if n < 2 {
            return HashSet::new();
        }
        
        let mut sieve = vec![true; (n + 1) as usize];
        sieve[0] = false;
        sieve[1] = false;
        
        let limit = (n as f64).sqrt() as u32;
        for i in 2..=limit {
            if sieve[i as usize] {
                let mut j = i * i;
                while j <= n {
                    sieve[j as usize] = false;
                    j += i;
                }
            }
        }
        
        (2..=n).filter(|&i| sieve[i as usize]).collect()
    }
    
    fn generate_family(&self, base_number: u32, replacement_positions: &[usize]) -> Vec<u32> {
        let base_str = base_number.to_string();
        let base_chars: Vec<char> = base_str.chars().collect();
        let mut family = Vec::new();
        
        for replacement_digit in 0..=9 {
            let mut new_chars = base_chars.clone();
            
            for &pos in replacement_positions {
                if pos < new_chars.len() {
                    new_chars[pos] = char::from_digit(replacement_digit, 10).unwrap();
                }
            }
            
            if new_chars.len() > 1 && new_chars[0] == '0' {
                continue;
            }
            
            if let Ok(new_number) = new_chars.iter().collect::<String>().parse::<u32>() {
                family.push(new_number);
            }
        }
        
        family
    }
    
    fn analyze_family(&self, family: &[u32]) -> FamilyInfo {
        let primes: Vec<u32> = family.iter()
            .copied()
            .filter(|&num| self.prime_set.contains(&num))
            .collect();
        
        let smallest_prime = primes.iter().min().copied();
        
        FamilyInfo {
            prime_count: primes.len(),
            smallest_prime,
        }
    }
    
    fn get_or_compute_family(&mut self, base_number: u32, replacement_positions: &[usize]) -> FamilyInfo {
        let pattern_key = PatternKey::new(base_number, replacement_positions);
        
        if let Some(cached_info) = self.pattern_cache.get(&pattern_key) {
            self.cache_hits += 1;
            return cached_info.clone();
        }
        
        self.cache_misses += 1;
        let family = self.generate_family(base_number, replacement_positions);
        let family_info = self.analyze_family(&family);
        
        self.pattern_cache.insert(pattern_key, family_info.clone());
        
        family_info
    }
    
    fn get_replacement_patterns(&self, number: u32, max_positions: usize) -> Vec<Vec<usize>> {
        let num_digits = number.to_string().len();
        let mut patterns = Vec::new();
        
        for num_positions in 1..=max_positions.min(num_digits) {
            self.generate_combinations(0, num_digits, num_positions, &mut Vec::new(), &mut patterns);
        }
        
        patterns
    }
    
    fn generate_combinations(
        &self,
        start: usize,
        total: usize,
        remaining: usize,
        current: &mut Vec<usize>,
        results: &mut Vec<Vec<usize>>
    ) {
        if remaining == 0 {
            results.push(current.clone());
            return;
        }
        
        if start >= total {
            return;
        }
        
        current.push(start);
        self.generate_combinations(start + 1, total, remaining - 1, current, results);
        current.pop();
        
        self.generate_combinations(start + 1, total, remaining, current, results);
    }
    
    fn find_target_prime_family(&mut self, target_prime_count: usize) -> Option<u32> {
        const MAX_REPLACEMENT_POSITIONS: usize = 3;
        
        let primes_clone = self.primes_list.clone();
        for prime in primes_clone {
            let replacement_patterns = self.get_replacement_patterns(prime, MAX_REPLACEMENT_POSITIONS);
            
            for pattern in replacement_patterns {
                let family_info = self.get_or_compute_family(prime, &pattern);
                
                if family_info.prime_count == target_prime_count {
                    return family_info.smallest_prime;
                }
            }
        }
        
        None
    }
}

impl Solution for Problem051 {
    fn number(&self) -> u32 {
        51
    }

    fn title(&self) -> &'static str {
        "Prime Digit Replacements"
    }

    fn solve(&self) -> String {
        let mut finder = PrimeFamilyFinder::new(1_000_000);
        
        match finder.find_target_prime_family(8) {
            Some(smallest_prime) => smallest_prime.to_string(),
            None => "No 8-prime family found in range".to_string(),
        }
    }
}
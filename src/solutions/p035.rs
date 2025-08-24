use crate::solutions::Solution;

pub struct Problem035;

impl Problem035 {
    pub fn new() -> Self {
        Self
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=f64::from(n).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// can be optimized by using a sieve to find all primes in the range and then check rotations
// or have a bitset to keep track of computed circular primes (ie, larger ones found by rotating smaller ones) so we don't re-check once we get to them

impl Solution for Problem035 {
    fn number(&self) -> u32 { 35 }

    fn title(&self) -> &'static str {
        "Circular Primes"
    }

    fn solve(&self) -> String {

        let mut counter = 0;

        for i in 2..=1_000_000 {
            if is_prime(i) {
                // Check if the prime is circular
                let mut is_circular = true;
                let s = i.to_string();
                for j in 0..s.len() {
                    let rotated = s[j..].to_string() + &s[..j];
                    if !is_prime(rotated.parse().unwrap()) {
                        is_circular = false;
                        break;
                    }
                }
                if is_circular {
                    counter += 1;
                }
            }
        }

        counter.to_string()
    }
}
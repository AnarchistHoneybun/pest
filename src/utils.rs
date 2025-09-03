pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn prime_sieve(lower: u64, upper: u64) -> Vec<u64> {
    if upper < 2 || lower > upper {
        return Vec::new();
    }
    
    let sieve_upper = upper + 1;
    let mut sieve = vec![true; sieve_upper as usize];
    sieve[0] = false;
    if sieve.len() > 1 {
        sieve[1] = false;
    }
    
    let limit = (sieve_upper as f64).sqrt() as u64;
    for i in 2..=limit {
        if sieve[i as usize] {
            let mut j = i * i;
            while j < sieve_upper {
                sieve[j as usize] = false;
                j += i;
            }
        }
    }
    
    (lower.max(2)..sieve_upper)
        .filter(|&i| sieve[i as usize])
        .collect()
}

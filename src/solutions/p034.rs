use crate::solutions::Solution;

pub struct Problem034;

impl Problem034 {
    pub fn new() -> Self {
        Self
    }
    
    fn factorial(n: u32) -> u32 {
        match n {
            0 | 1 => 1,
            2 => 2,
            3 => 6,
            4 => 24,
            5 => 120,
            6 => 720,
            7 => 5040,
            8 => 40320,
            9 => 362880,
            _ => 0,
        }
    }
    
    fn sum_of_digit_factorials(mut n: u32) -> u32 {
        let mut sum = 0;
        while n > 0 {
            sum += Self::factorial(n % 10);
            n /= 10;
        }
        sum
    }
    
    fn find_upper_bound() -> u32 {
        let nine_factorial = Self::factorial(9);
        let mut digits = 1u32;
        
        loop {
            let max_sum_for_n_digits = digits * nine_factorial;
            let min_n_digit_number = 10u32.pow(digits - 1);
            
            if max_sum_for_n_digits < min_n_digit_number {
                return (digits - 1) * nine_factorial;
            }
            
            digits += 1;
        }
    }
}

impl Solution for Problem034 {
    fn number(&self) -> u32 { 34 }

    fn title(&self) -> &'static str {
        "Digit Factorials"
    }

    fn solve(&self) -> String {
        let upper_bound = Self::find_upper_bound();
        
        let mut sum = 0;
        
        for i in 3..=upper_bound {
            if i == Self::sum_of_digit_factorials(i) {
                sum += i;
            }
        }
        
        sum.to_string()
    }
}
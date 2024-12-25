use crate::solutions::Solution;

pub struct Problem017;

impl Problem017 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem017 {
    fn number(&self) -> u32 { 17 }

    fn title(&self) -> &'static str {
        "Number letter counts"
    }

    fn solve(&self) -> String {
        let mut sum = 0;
        for i in 1..=1000 {
            sum += number_word_len(i);
        }
        sum.to_string()
    }
}
const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen"
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

fn to_english(n: i32) -> String {
    if n < 0 || n >= 1_000_000 {
        panic!("Number out of supported range");
    }

    if n < 20 {
        return ONES[n as usize].to_string();
    }

    if n < 100 {
        let tens = TENS[n as usize / 10].to_string();
        let ones = if n % 10 != 0 {
            ONES[(n % 10) as usize].to_string()
        } else {
            String::new()
        };
        return format!("{}{}", tens, ones);
    }

    if n < 1000 {
        let hundreds = format!("{}{}", ONES[(n / 100) as usize], "hundred");
        if n % 100 != 0 {
            return format!("{}and{}", hundreds, to_english(n % 100));
        }
        return hundreds;
    }

    // Handle thousands
    let thousands = format!("{}thousand", to_english(n / 1000));
    if n % 1000 != 0 {
        return format!("{}{}", thousands, to_english(n % 1000));
    }
    thousands
}

pub fn number_word_len(n: i32) -> usize {
    to_english(n).len()
}
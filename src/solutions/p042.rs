use std::io::BufRead;

use crate::solutions::Solution;


pub struct Problem042;

impl Problem042 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem042 {
    fn number(&self) -> u32 {
        42
    }

    fn title(&self) -> &'static str {
        "Coded Triangle Numbers"
    }

    fn solve(&self) -> String {
        let file = std::fs::File::open("resources/p042_words.txt").unwrap();

        let words = std::io::BufReader::new(file).split(b',').map(|r| {
            let bytes = r.unwrap();
            String::from_utf8(bytes.into_iter().filter(|&b| b != b'"').collect()).unwrap()
        }).collect::<Vec<_>>();

        let max_word_length = words.iter().map(|w| w.len()).max().unwrap_or(0);
        let upper_limit = max_word_length * 26;

        let triangle_numbers: Vec<u32> = (1..)
            .map(|n| n * (n + 1) / 2)
            .take_while(|&n| n <= upper_limit as u32)
            .collect();

        let count = words.iter().filter(|word| {
            let value = word.chars().map(|c| c as u32 - 64).sum::<u32>();
            triangle_numbers.contains(&value)
        }).count();

        count.to_string()
    }
}

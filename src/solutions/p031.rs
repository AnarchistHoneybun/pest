use crate::solutions::Solution;

pub struct Problem031;

impl Problem031 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem031 {
    fn number(&self) -> u32 { 31 }

    fn title(&self) -> &'static str {
        "Coin Sums"
    }

    fn solve(&self) -> String {
        const TOTAL: usize = 200;

        let mut combs: Vec<u64> = vec![0; TOTAL + 1];
        combs[0] = 1;

        let currencies = [1, 2, 5, 10, 20, 50, 100, 200];

        for currency in currencies {
            for i in 0..=TOTAL- currency {
                combs[i + currency] += combs[i];
            }
        }

        combs[TOTAL].to_string()

    }
}
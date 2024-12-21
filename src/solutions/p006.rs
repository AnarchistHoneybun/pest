use crate::solutions::Solution;

pub struct Problem006;

impl Problem006 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem006 {
    fn number(&self) -> u32 { 6 }

    fn title(&self) -> &'static str {
        "Sum square difference"
    }

    fn solve(&self) -> String {
        let sum_of_squares: u32 = (1..=100).map(|x| x * x).sum();
        let square_of_sum: u32 = (1..=100).sum::<u32>().pow(2);
        (square_of_sum - sum_of_squares).to_string()
    }
}
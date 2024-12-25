use crate::solutions::Solution;

pub struct Problem019;

impl Problem019 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem019 {
    fn number(&self) -> u32 { 19 }

    fn title(&self) -> &'static str {
        "Counting Sundays"
    }

    fn solve(&self) -> String {
        // check number of sundays from 1 Jan 1901 to 31 Dec 2000
        let mut count = 0;
        for year in 1901..=2000 {
            for month in 1..=12 {
                if zellers_congruence((year, month, 1)) == 1 {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}
fn zellers_congruence(date: (u32, u32, u32)) -> u32 {
    let (year, month, day) = date;
    let mut m = month;
    let mut y = year;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;
    (day + 13 * (m + 1) / 5 + k + k / 4 + j / 4 + 5 * j) % 7 + 1
}
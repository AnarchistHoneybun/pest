use crate::solutions::Solution;

pub struct Problem045;

impl Problem045 {
    pub fn new() -> Self {
        Self
    }
}

fn is_pentagonal(num: u64) -> bool {
    let discriminant = 24 * num + 1;
    let sqrt = (discriminant as f64).sqrt();
    if sqrt.fract() != 0.0 {
        return false;
    }
    let n = (1.0 + sqrt) / 6.0;
    n.fract() == 0.0 && n > 0.0
}

fn is_hexagonal(num: u64) -> bool {
    let discriminant = 8 * num + 1;
    let sqrt = (discriminant as f64).sqrt();
    if sqrt.fract() != 0.0 {
        return false;
    }
    let n = (1.0 + sqrt) / 4.0;
    n.fract() == 0.0 && n > 0.0
}

impl Solution for Problem045 {
    fn number(&self) -> u32 { 45 }

    fn title(&self) -> &'static str {
        "Triangular, Pentagonal, and Hexagonal"
    }

    fn solve(&self) -> String {
        let mut n = 286;
        loop {
            let triangle = n * (n + 1) / 2;
            if is_pentagonal(triangle) && is_hexagonal(triangle) {
                return triangle.to_string();
            }
            n += 1;
        }
    }
}
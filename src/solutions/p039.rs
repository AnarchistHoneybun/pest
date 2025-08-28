use crate::solutions::Solution;

pub struct Problem039;

impl Problem039 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem039 {
    fn number(&self) -> u32 {
        39
    }

    fn title(&self) -> &'static str {
        "Integer Right Triangles"
    }

    fn solve(&self) -> String {
        let mut max_solutions = 0;
        let mut result_p = 0;
        for p in 1..1000 {
            let mut count = 0;
            for a in 1..p {
                for b in a..p - a {
                    let c = p - a - b;
                    if a * a + b * b == c * c {
                        count += 1;
                    }
                }
            }
            if count > max_solutions {
                max_solutions = count;
                result_p = p;
            }
        }
        result_p.to_string()
    }
}

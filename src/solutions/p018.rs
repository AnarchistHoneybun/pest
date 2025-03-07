use crate::solutions::Solution;

pub struct Problem018;

impl Problem018 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem018 {
    fn number(&self) -> u32 { 18 }

    fn title(&self) -> &'static str {
        "Maximum Path Sum I"
    }

    fn solve(&self) -> String {
        let triangle = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 04, 82, 47, 65],
            vec![19, 01, 23, 75, 03, 34],
            vec![88, 02, 77, 73, 07, 63, 67],
            vec![99, 65, 04, 28, 06, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
        ];
        let mut max_sum = triangle.clone();
        for i in 1..triangle.len() {
            max_sum[i][0] += max_sum[i - 1][0];
            max_sum[i][i] += max_sum[i - 1][i - 1];
            for j in 1..i {
                max_sum[i][j] += max_sum[i - 1][j - 1].max(max_sum[i - 1][j]);
            }
        }

        max_sum.last().unwrap().iter().max().unwrap().to_string()
    }
}
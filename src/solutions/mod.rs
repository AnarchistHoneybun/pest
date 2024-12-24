use std::time::Instant;
use std::collections::HashMap;

pub trait Solution {
    fn number(&self) -> u32;
    fn title(&self) -> &'static str;
    fn solve(&self) -> String;
}

pub fn run_solution(solution: &dyn Solution) {
    println!("Problem {:03}: {}", solution.number(), solution.title());

    let start = Instant::now();
    let answer = solution.solve();
    let duration = start.elapsed();

    println!("Answer: {}", answer);
    println!("Time: {:?}\n", duration);
}

#[macro_export]
macro_rules! register_solution {
    ($solutions:expr, $problem:ty) => {
        $solutions.insert(
            <$problem>::new().number(),
            Box::new(<$problem>::new()) as Box<dyn Solution>
        );
    };
}

pub struct SolutionFactory {
    solutions: HashMap<u32, Box<dyn Solution>>,
}

impl SolutionFactory {
    pub fn new() -> Self {
        let mut solutions = HashMap::new();
        register_solution!(solutions, p007::Problem007);
        register_solution!(solutions, p009::Problem009);
        register_solution!(solutions, p016::Problem016);
        register_solution!(solutions, p011::Problem011);
        register_solution!(solutions, p004::Problem004);
        register_solution!(solutions, p012::Problem012);
        register_solution!(solutions, p013::Problem013);
        register_solution!(solutions, p005::Problem005);
        register_solution!(solutions, p006::Problem006);
        register_solution!(solutions, p014::Problem014);
        register_solution!(solutions, p002::Problem002);
        register_solution!(solutions, p010::Problem010);
        register_solution!(solutions, p015::Problem015);
        register_solution!(solutions, p003::Problem003);
        register_solution!(solutions, p008::Problem008);
        register_solution!(solutions, p001::Problem001);

        Self { solutions }
    }

    pub fn run(&self, problems: Option<Vec<u32>>) {
        match problems {
            Some(nums) => {
                for num in nums {
                    if let Some(solution) = self.solutions.get(&num) {
                        run_solution(solution.as_ref());
                    } else {
                        println!("Solution for problem {} not found", num);
                    }
                }
            }
            None => {
                let mut numbers: Vec<_> = self.solutions.keys().collect();
                numbers.sort();
                for &num in numbers {
                    run_solution(self.solutions.get(&num).unwrap().as_ref());
                }
            }
        }
    }
}

pub mod p007;
pub mod p009;
pub mod p016;
pub mod p011;
pub mod p004;
pub mod p012;
pub mod p013;
pub mod p005;
pub mod p006;
pub mod p014;
pub mod p002;
pub mod p010;
pub mod p015;
pub mod p003;
pub mod p008;
pub mod p001;

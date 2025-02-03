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
        register_solution!(solutions, p032::Problem032);
        register_solution!(solutions, p029::Problem029);
        register_solution!(solutions, p020::Problem020);
        register_solution!(solutions, p027::Problem027);
        register_solution!(solutions, p022::Problem022);
        register_solution!(solutions, p009::Problem009);
        register_solution!(solutions, p016::Problem016);
        register_solution!(solutions, p011::Problem011);
        register_solution!(solutions, p018::Problem018);
        register_solution!(solutions, p004::Problem004);
        register_solution!(solutions, p028::Problem028);
        register_solution!(solutions, p023::Problem023);
        register_solution!(solutions, p025::Problem025);
        register_solution!(solutions, p021::Problem021);
        register_solution!(solutions, p012::Problem012);
        register_solution!(solutions, p026::Problem026);
        register_solution!(solutions, p013::Problem013);
        register_solution!(solutions, p005::Problem005);
        register_solution!(solutions, p006::Problem006);
        register_solution!(solutions, p017::Problem017);
        register_solution!(solutions, p014::Problem014);
        register_solution!(solutions, p002::Problem002);
        register_solution!(solutions, p024::Problem024);
        register_solution!(solutions, p010::Problem010);
        register_solution!(solutions, p015::Problem015);
        register_solution!(solutions, p033::Problem033);
        register_solution!(solutions, p003::Problem003);
        register_solution!(solutions, p019::Problem019);
        register_solution!(solutions, p030::Problem030);
        register_solution!(solutions, p031::Problem031);
        register_solution!(solutions, p008::Problem008);
        register_solution!(solutions, p001::Problem001);

        Self { solutions }
    }

    pub fn get_latest_problem_number(&self) -> u32 {
        *self.solutions.keys().max().unwrap_or(&0)
    }

    pub fn get_latest_n_problem_numbers(&self, n: usize) -> Vec<u32> {
        let mut numbers: Vec<_> = self.solutions.keys().copied().collect();
        numbers.sort_by(|a, b| b.cmp(a));  // Sort in descending order
        numbers.truncate(n);
        numbers.sort();  // Sort in ascending order for display
        numbers
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
pub mod p032;
pub mod p029;
pub mod p020;
pub mod p027;
pub mod p022;
pub mod p009;
pub mod p016;
pub mod p011;
pub mod p018;
pub mod p004;
pub mod p028;
pub mod p023;
pub mod p025;
pub mod p021;
pub mod p012;
pub mod p026;
pub mod p013;
pub mod p005;
pub mod p006;
pub mod p017;
pub mod p014;
pub mod p002;
pub mod p024;
pub mod p010;
pub mod p015;
pub mod p033;
pub mod p003;
pub mod p019;
pub mod p030;
pub mod p031;
pub mod p008;
pub mod p001;

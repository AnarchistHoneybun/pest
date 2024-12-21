use std::fs;
use std::path::Path;
use std::io::Write;

fn main() {
    let solutions_dir = Path::new("src/solutions");
    let mod_file = solutions_dir.join("mod.rs");

    // Read all .rs files in the solutions directory
    let mut modules = Vec::new();
    if let Ok(entries) = fs::read_dir(solutions_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                    if let Some(stem) = path.file_stem() {
                        if stem != "mod" {
                            modules.push(stem.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }

    // Create the base content of mod.rs
    let mut content = String::from(r#"use std::time::Instant;
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
"#);

    // Add solution registrations
    for module in &modules {
        content.push_str(&format!(
            "        register_solution!(solutions, {}::Problem{});\n",
            module,
            module.replace("p", "").replace("_", "")
        ));
    }

    content.push_str(r#"
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

"#);

    // Add module declarations
    for module in &modules {
        content.push_str(&format!("pub mod {};\n", module));
    }

    // Write the updated content to mod.rs
    if let Ok(mut file) = fs::File::create(&mod_file) {
        file.write_all(content.as_bytes()).unwrap();
    }
}
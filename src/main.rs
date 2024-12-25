use std::env;
mod solutions;
use solutions::SolutionFactory;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let factory = SolutionFactory::new();

    let problems = if !args.is_empty() {
        if args[0] == "--all" {
            None
        } else if args[0] == "--recent" {
            Some(vec![factory.get_latest_problem_number()])
        } else if args[0] == "--n" {
            if args.len() < 2 {
                println!("Please specify the number of recent problems to run after --n");
                return;
            }
            match args[1].parse::<usize>() {
                Ok(n) => Some(factory.get_latest_n_problem_numbers(n)),
                Err(_) => {
                    println!("Invalid number provided after --n");
                    return;
                }
            }
        } else if args[0].starts_with("--problems=") {
            let numbers_str = args[0].trim_start_matches("--problems=");
            let problems: Vec<u32> = numbers_str
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            if problems.is_empty() {
                println!("No valid problem numbers provided. Use format: --problems=1,2,3");
                return;
            }
            Some(problems)
        } else {
            println!("Unknown argument. Available options:");
            println!("  --all          Run all solutions");
            println!("  --recent       Run most recent solution");
            println!("  --n <number>   Run last N solutions");
            println!("  --problems=X,Y,Z  Run specific problems (comma-separated)");
            return;
        }
    } else {
        Some(factory.get_latest_n_problem_numbers(3))
    };

    factory.run(problems);
}
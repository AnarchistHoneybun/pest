use std::env;
mod solutions;
use solutions::SolutionFactory;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let factory = SolutionFactory::new();

    let problems = if !args.is_empty() {
        Some(args
            .iter()
            .filter_map(|arg| arg.parse::<u32>().ok())
            .collect())
    } else {
        None
    };

    factory.run(problems);
}
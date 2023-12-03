use aoc_2023::{all_days, get_input, print_results_table};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "aoc", about = "Joey9801's advent-of-code solutions")]
struct Opt {
    /// Only run the given day
    #[arg(name = "DAY", long = "day")]
    day: Option<u8>,

    #[arg(name = "INPUT_ROOT", long = "input_root", default_value = "./inputs")]
    input_root: PathBuf,
}

fn main() {
    let opt = Opt::parse();
    let mut solutions = all_days();

    if let Some(day) = &opt.day {
        solutions = solutions
            .drain(..)
            .filter(|d| d.name().day == *day)
            .collect();
    }

    if solutions.len() == 0 {
        println!("No solutions match CLI opts: {:?}", &opt);
    } else {
        let results = solutions
            .iter()
            .map(|d| {
                let input = get_input(&opt.input_root, d.name()).expect("Failed to find an input");
                d.run(&input)
            })
            .collect::<Vec<_>>();

        print_results_table(&results);
    }
}

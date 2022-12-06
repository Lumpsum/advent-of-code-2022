use advent_of_code::{*};
use clap::Parser;
use std::panic;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    day: u8,

    /// Number of times to greet
    #[arg(short, long)]
    puzzle: u8,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let day_puzzle = (args.day, args.puzzle);

    let result = match day_puzzle {
        (1, 1) => day_1_puzzle_1(),
        (1, 2) => day_1_puzzle_2(),
        (2, 1) => day_2_puzzle_1(),
        (2, 2) => day_2_puzzle_2(),
        (3, 1) => day_3_puzzle_1(),
        (3, 2) => day_3_puzzle_2(),
        (4, 1) => day_4_puzzle_1(),
        (4, 2) => day_4_puzzle_2(),
        (5, 1) => day_5_puzzle_1(),
        (5, 2) => day_5_puzzle_2(),
        (6, 1) => day_6_puzzle_1(),
        (6, 2) => day_6_puzzle_2(),
        _ => panic!("{}", format!("No implementation for day {} puzzle {}", day_puzzle.0, day_puzzle.1))
    };

    println!("{:?}", result);
    Ok(())
}

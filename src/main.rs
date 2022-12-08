use advent_of_code::{*};
use clap::Parser;
use std::panic;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: u8,

    // #[arg(short, long)]
    // puzzle: u8,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let result = match args.day {
        1 => run_day_usize(1),
        2 => run_day_usize(2),
        // (1, 2) => day_1_puzzle_2(),
        // (2, 1) => day_2_puzzle_1(),
        // (2, 2) => day_2_puzzle_2(),
        // (3, 1) => day_3_puzzle_1(),
        // (3, 2) => day_3_puzzle_2(),
        // (4, 1) => day_4_puzzle_1(),
        // (4, 2) => day_4_puzzle_2(),
        // (5, 1) => day_5_puzzle_1(),
        // (5, 2) => day_5_puzzle_2(),
        // (6, 1) => day_6_puzzle_1(),
        // (6, 2) => day_6_puzzle_2(),
        _ => panic!("{}", format!("No implementation for day {}", args.day))
    };

    match result {
        Some(i) => println!("{:?}", i),
        None => panic!("Invalid outcome")
    }
    Ok(())
}

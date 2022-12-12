use day_1::DayOne;
use day_2::DayTwo;
use day_3::DayThree;
use day_4::DayFour;
use day_5::DayFive;
use day_6::DaySix;
use day_7::DaySeven;
use solution::{Solution, Outcome};

mod constants;
mod solution;
mod reader;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;


pub enum PuzzleResult {
    Number(Option<Outcome<usize>>),
    Text(Option<Outcome<String>>)
}


pub fn run_day_usize(day: usize) -> PuzzleResult {
    let result = match day {
        1 => DayOne::run(1),
        2 => DayTwo::run(2),
        3 => DayThree::run(3),
        4 => DayFour::run(4),
        6 => DaySix::run(6),
        7 => DaySeven::run(7),
        _ => panic!("Day not implemented")
    };
    PuzzleResult::Number(result)
}

pub fn run_day_string(day: usize) -> PuzzleResult {
    let result = match day {
        5 => DayFive::run(5),
        _ => panic!("Day not implemented")
    };
    PuzzleResult::Text(result)
}

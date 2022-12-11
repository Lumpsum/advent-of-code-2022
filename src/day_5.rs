use crate::solution::Solution;

pub struct DayFive {}


impl Solution for DayFive {
    type Input = String;
    type Output = String;

    fn preprocess(data: &str) -> Self::Input {
        "".to_string()
    }

    fn puzzle_one(input: &Self::Input) -> Option<Self::Output> {
        None
    }

    fn puzzle_two(input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

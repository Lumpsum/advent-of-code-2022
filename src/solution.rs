use std::fmt::Debug;

use crate::constants;

pub struct Outcome<T> {
    puzzle_1: Option<T>,
    puzzle_2: Option<T>
}

impl<T> std::fmt::Debug for Outcome<T>
where 
    T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.puzzle_1 {
            Some(i) => writeln!(f, "Puzzle 1: {:?}", i).expect("Could not write!"),
            None => writeln!(f, "Puzzle 1 not implemented").expect("Could not write!")
        };
        match &self.puzzle_2 {
            Some(i) => writeln!(f, "Puzzle 2: {:?}", i).expect("Could not write!"),
            None => writeln!(f, "Puzzle 2 not implemented").expect("Could not write!")
        };
        Ok(())
    }
}


pub trait Solution {
    type Input<'a>;
    type Output: Debug;

    fn load_data(day: usize) -> String {
        let data: String = constants::READER.read(day).expect("Could not read file");
        data
    }

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a>;

    #[allow(unused_variables)]
    fn puzzle_one<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        None
    }

    #[allow(unused_variables)]
    fn puzzle_two<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        None
    }

    fn run(day: usize) -> Option<Outcome<Self::Output>> {
        let data = Self::load_data(day);
        let preprocess_data = Self::preprocess(&data);

        let outcome = Outcome {
            puzzle_1: Self::puzzle_one(&preprocess_data),
            puzzle_2: Self::puzzle_two(&preprocess_data)
        };
        Some(outcome)
    }
}

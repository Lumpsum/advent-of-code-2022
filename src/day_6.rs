use std::collections::HashSet;

use crate::solution::Solution;

pub struct DaySix {}


fn get_first_unique_set(input: &Vec<char>, set_size: usize) -> Option<usize> {
    for window in input.windows(set_size).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window.1);
        if set.len() == set_size {
            return Some(window.0 + set_size)
        }
    }
    None
}


impl Solution for DaySix {
    type Input<'a> = Vec<char>;
    type Output = usize;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        data.chars().into_iter().collect::<Vec<char>>()
    }

    fn puzzle_one<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        get_first_unique_set(input, 4)
    }

    fn puzzle_two<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        get_first_unique_set(input, 14)
    }
}

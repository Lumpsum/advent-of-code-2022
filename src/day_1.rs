use crate::solution::Solution;

pub struct DayOne {}


impl Solution for DayOne {
    type Input<'a> = Vec<usize>;
    type Output = usize;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        let input: Vec<Option<usize>> = data.lines().map(|i| str::parse::<usize>(i)).map(Result::ok).collect();
        let mut list_of_sum: Vec<usize> = input
            .split(|i| i.is_none())
            .map(
                |list| list.into_iter()
                .filter_map(|&i: &Option<usize>| i)
                .sum()
            ).collect();
        list_of_sum.sort();
        list_of_sum
    }

    fn puzzle_one<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let answer = input.iter().max().unwrap();
        Some(*answer)
    }

    fn puzzle_two<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let answer = input.iter().rev().take(3).sum();
        Some(answer)
    }
}

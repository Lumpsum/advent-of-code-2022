use std::{str::FromStr, collections::HashSet};

use crate::solution::Solution;

pub struct DayThree {}

#[derive(Debug, Clone)]
pub struct RuckSack {
    first_half: Vec<char>,
    second_half: Vec<char>
}

impl RuckSack {
    fn find_common_elements(&self) -> char {
        let first_set: HashSet<char> = HashSet::from_iter(self.first_half.clone());
        let second_set: HashSet<char> = HashSet::from_iter(self.second_half.clone());

        let intersection = first_set.intersection(&second_set).collect::<Vec<&char>>();
        intersection[0].clone()
    }

    fn get_all_unique_supplies(&self) -> HashSet<char> {
        let mut all_supplies: HashSet<char> = HashSet::from_iter(self.first_half.clone());
        all_supplies.extend(self.second_half.clone());
        all_supplies
    }
}

impl FromStr for RuckSack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_s = s.split_at(s.len() / 2);
        Ok(RuckSack { 
            first_half: split_s.0.chars().collect(),
            second_half: split_s.1.chars().collect() 
        })
    }
}

struct Priority(usize);

impl TryFrom<char> for Priority {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let priority = match value {
            c if c >= 'a' && c <= 'z' => Ok((c as usize) - 96),
            c2 if c2 >= 'A' && c2 <= 'Z' => Ok((c2 as usize) - 38),
            invalid_char => Err(format!("Invalid character: {}", invalid_char))
        };
        Ok(Priority(priority.unwrap()))
    }
}

fn find_common_elements(chunk: &[RuckSack]) -> char {
    let mut r = chunk[0].get_all_unique_supplies();

    for sack in &chunk[1..] {
        r = HashSet::from_iter(r.intersection(&sack.get_all_unique_supplies()).into_iter().map(|&i| i));
    }
    r.iter().collect::<Vec<&char>>()[0].clone()
}

impl Solution for DayThree {
    type Input<'a> = Vec<RuckSack>;
    type Output = usize;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        data.lines().flat_map(|line| RuckSack::from_str(line).ok()).collect::<Self::Input<'a>>()
    }

    fn puzzle_one<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let answer = input.iter().map(|r| r.find_common_elements()).filter_map(|c| Priority::try_from(c).ok()).collect::<Vec<_>>();
        Some(answer.iter().map(|priority| priority.0).sum())
    }

    fn puzzle_two<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let answer = input.chunks(3).map(|chunk| find_common_elements(chunk)).filter_map(|c| Priority::try_from(c).ok()).collect::<Vec<_>>();
        Some(answer.iter().map(|priority| priority.0).sum())
    }
}

use std::{str::FromStr, collections::HashSet};

use crate::solution::Solution;

pub struct DayFour {}


pub struct SectionAssignmentPair {
    left: Vec<usize>,
    right: Vec<usize>
}


impl FromStr for SectionAssignmentPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_string: Vec<&str> = s.split(",").collect();

        let value = split_string
            .iter()
            .map(|&s| s.split("-").collect())
            .map(|v: Vec<&str>| (str::parse::<usize>(v[0]).unwrap()..(str::parse::<usize>(v[1]).unwrap() + 1)).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        Ok(SectionAssignmentPair {
            left: value[0].clone(),
            right: value[1].clone()
        })
    }
}


impl Solution for DayFour {
    type Input = Vec<SectionAssignmentPair>;
    type Output = usize;

    fn preprocess(data: &str) -> Self::Input {
        data.lines().flat_map(|line| SectionAssignmentPair::from_str(line)).collect::<Vec<SectionAssignmentPair>>()
    }

    fn puzzle_one(input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().filter_map(|line| {
            let left_hashset: HashSet<usize> = HashSet::from_iter(line.left.clone());
            let right_hashset: HashSet<usize> = HashSet::from_iter(line.right.clone());

            if left_hashset.is_subset(&right_hashset) || left_hashset.is_superset(&right_hashset) {
                Some(line)
            } else {
                None
            }
        }).count())
    }

    fn puzzle_two(input: &Self::Input) -> Option<Self::Output> {
        Some(input.iter().filter_map(|line| {
            if line.left.contains(&line.right[0]) || line.right.contains(&line.left[0]) {
                Some(line)
            } else {
                None
            }
        }).count())
    }
}

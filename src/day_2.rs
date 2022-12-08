use std::{str::FromStr, cmp::Ordering};

use crate::solution::Solution;

pub struct DayTwo {}

#[derive(Eq, Debug, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<char> for Hand {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Hand::Rock),
            'B' => Ok(Hand::Paper),
            'C' => Ok(Hand::Scissor),

            'X' => Ok(Hand::Rock),
            'Y' => Ok(Hand::Paper),
            'Z' => Ok(Hand::Scissor),

            c => Err(format!("Invalid character: '{}'",c))
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::Rock, Hand::Paper) => Ordering::Less,
            (Hand::Rock, Hand::Scissor) => Ordering::Greater,
            (Hand::Rock, Hand::Rock) => Ordering::Equal,

            (Hand::Paper, Hand::Rock) => Ordering::Greater,
            (Hand::Paper, Hand::Scissor) => Ordering::Less,
            (Hand::Paper, Hand::Paper) => Ordering::Equal,

            (Hand::Scissor, Hand::Rock) => Ordering::Less,
            (Hand::Scissor, Hand::Paper) => Ordering::Greater,
            (Hand::Scissor, Hand::Scissor) => Ordering::Equal,
        }
    }
}

pub struct Play {
    opponent: Hand,
    player: Hand
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissor),
            "D" => Ok(Hand::Rock),
            "E" => Ok(Hand::Paper),
            "F" => Ok(Hand::Scissor),
            c => Err(format!("Invalid character: {}", c))
        }
    }
}

impl FromStr for Play {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hands = s.chars().into_iter().filter_map(|c| Hand::try_from(c).ok());
        Ok(Play {
            opponent: hands.next().unwrap(),
            player: hands.last().unwrap(),
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Score(usize);

impl From<&Play> for Score {
    fn from(play: &Play) -> Self {
        let hand_score: usize = match play.player {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3
        };

        let game_score: usize = match play.player.cmp(&play.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        Self(hand_score + game_score)
    }
}


impl Solution for DayTwo {
    type Input = Vec<Play>;
    type Output = usize;

    fn preprocess(data: &str) -> Self::Input {
        data.lines().filter_map(|line| Play::from_str(line).ok()).collect()
    }

    fn puzzle_one(input: &Self::Input) -> Option<Self::Output> {
        let answer = input.into_iter().map(|play| Score::from(play)).collect::<Vec<_>>();
        Some(answer.iter().map(|score| score.0).sum())
    }

    fn puzzle_two(input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

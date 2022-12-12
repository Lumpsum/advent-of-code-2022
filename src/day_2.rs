use std::{str::FromStr, cmp::Ordering};

use crate::solution::Solution;

pub struct DayTwo {}

impl DayTwo {
    fn sum_score(vec_score: Vec<Score>) -> usize {
        vec_score.iter().map(|score| score.0).sum()
    }

    fn play_into_score<'a, T>(iterator: T) -> Vec<Score>
    where
        T: Iterator<Item = &'a Play>
    {
        iterator.map(|play| Score::from(play)).collect::<Vec<_>>()
    }
}

#[derive(Eq, Debug, PartialEq, Copy, Clone)]
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

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissor),

            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scissor),
            c => Err(format!("Invalid character: {}", c))
        }
    }
}

pub struct Play {
    opponent: Hand,
    player: Hand
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

impl From<&Strategy> for Play {
    fn from(strat: &Strategy) -> Self {
        let play = match (strat.opponent, strat.result) {
            (_, PlayResult::Draw) => strat.opponent,

            (Hand::Rock, PlayResult::Win) => Hand::Paper,
            (Hand::Rock, PlayResult::Loss) => Hand::Scissor,

            (Hand::Paper, PlayResult::Win) => Hand::Scissor,
            (Hand::Paper, PlayResult::Loss) => Hand::Rock,

            (Hand::Scissor, PlayResult::Win) => Hand::Rock,
            (Hand::Scissor, PlayResult::Loss) => Hand::Paper,
        };
        Play { opponent: strat.opponent, player: play }
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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum PlayResult {
    Win = 'Z' as isize,
    Draw = 'Y' as isize,
    Loss = 'X' as isize
}


#[derive(Eq, PartialEq, Debug)]
struct Strategy {
    opponent: Hand,
    result: PlayResult
}

impl From<&Play> for Strategy {
    fn from(play: &Play) -> Self {
        let result = match play.player {
            Hand::Rock => PlayResult::Loss,
            Hand::Paper => PlayResult::Draw,
            Hand::Scissor => PlayResult::Win
        };
        Strategy { opponent: play.opponent, result: result }
    }
}


impl Solution for DayTwo {
    type Input<'a> = Vec<Play>;
    type Output = usize;

    fn preprocess<'a>(data: &'a str) -> Self::Input<'a> {
        data.lines().filter_map(|line| Play::from_str(line).ok()).collect()
    }

    fn puzzle_one<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let answer = Self::play_into_score(input.into_iter());
        Some(Self::sum_score(answer))
    }

    fn puzzle_two<'a>(input: &Self::Input<'a>) -> Option<Self::Output> {
        let answer = input.into_iter().map(|line| Strategy::from(line)).map(|strat| Play::from(&strat)).map(|play| Score::from(&play)).collect::<Vec<_>>();
        Some(Self::sum_score(answer))
    }
}

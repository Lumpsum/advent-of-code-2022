use std::cmp::Ordering;
use crate::errors::StringConversionRPSError;

#[derive(Eq, Debug, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl RPS {
    fn get_int_value(&self) -> i32 {
        let score = match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        };
        score
    }

    fn get_score(&self, other: &Self) -> i32 {
        if self > other {
            return 6
        } else if self == other {
            return 3
        }
        0
    }

    pub fn get_result_pick(&self, result: &str) -> Result<RPS, StringConversionRPSError> {
        let winning_pick = match result {
            "X" => {
                match self {
                    RPS::Rock => Ok(RPS::Scissor),
                    RPS::Paper => Ok(RPS::Rock),
                    RPS::Scissor => Ok(RPS::Paper),
                }
            },
            "Y" => {
                match self {
                    RPS::Rock => Ok(RPS::Rock),
                    RPS::Paper => Ok(RPS::Paper),
                    RPS::Scissor => Ok(RPS::Scissor),
                }
            },
            "Z" => {
                match self {
                    RPS::Rock => Ok(RPS::Paper),
                    RPS::Paper => Ok(RPS::Scissor),
                    RPS::Scissor => Ok(RPS::Rock),
                }
            }
            &_ => Err(StringConversionRPSError)
        };
        winning_pick
    }

    pub fn get_total_score(&self, other: &Self) -> i32 {
        let mut score: i32 = self.get_int_value();
        score += self.get_score(&other);
        score
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RPS::Rock, RPS::Paper) => Ordering::Less,
            (RPS::Rock, RPS::Scissor) => Ordering::Greater,
            (RPS::Rock, RPS::Rock) => Ordering::Equal,
            (RPS::Paper, RPS::Rock) => Ordering::Greater,
            (RPS::Paper, RPS::Scissor) => Ordering::Less,
            (RPS::Paper, RPS::Paper) => Ordering::Equal,
            (RPS::Scissor, RPS::Rock) => Ordering::Less,
            (RPS::Scissor, RPS::Paper) => Ordering::Greater,
            (RPS::Scissor, RPS::Scissor) => Ordering::Equal,
        }
    }
}
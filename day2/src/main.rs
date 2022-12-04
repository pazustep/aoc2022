use std::str::FromStr;

use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;

trait Points {
    fn points(&self) -> usize;
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(eyre!("not a valid move: {value:?}")),
        }
    }
}

impl Points for Move {
    fn points(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl Move {
    fn with_outcome(self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (_, Outcome::Draw) => self,
            (Self::Rock, Outcome::Loss) => Self::Scissors,
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Paper, Outcome::Loss) => Self::Rock,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Scissors, Outcome::Loss) => Self::Paper,
            (Self::Scissors, Outcome::Win) => Self::Rock,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Points for Outcome {
    fn points(&self) -> usize {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = Report;

    fn try_from(c: char) -> Result<Self, Report> {
        match c {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(eyre!("not a valid outcome: {c}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            outcome: outcome.try_into()?,
        })
    }
}

impl Points for Round {
    fn points(&self) -> usize {
        let ours = self.theirs.with_outcome(self.outcome);
        ours.points() + self.outcome.points()
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let total_score: usize = itertools::process_results(
        include_str!("input.txt")
            .lines()
            .map(Round::from_str)
            .map_ok(|r| r.points()),
        |it| it.sum(),
    )?;

    dbg!(total_score);
    Ok(())
}

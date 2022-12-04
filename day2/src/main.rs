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
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
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
    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Points for Outcome {
    fn points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }
}

impl FromStr for Round {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Points for Round {
    fn points(&self) -> usize {
        self.ours.points() + self.outcome().points()
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

use nom::bytes::complete::tag;
use nom::character::complete::{anychar, u8};
use nom::combinator::map_opt;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

fn main() {
    let visited_part_1 = count_visited(2, include_str!("input.txt"));
    println!("visited positions (2 knots): {visited_part_1}");

    let visited_part_1 = count_visited(10, include_str!("input.txt"));
    println!("visited positions (10 knots): {visited_part_1}");
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        let zero = Point::default();
        let knots = vec![zero; knots];

        let mut visited = HashSet::new();
        visited.insert(zero);

        Self { knots, visited }
    }

    fn pull(&mut self, direction: Direction) {
        let knots = &mut self.knots;

        // move the head
        let mut previous = {
            let head = knots.get_mut(0).unwrap();
            *head = head.move_to(direction);
            *head
        };

        for knot in knots.iter_mut().skip(1) {
            match (previous.x - knot.x, previous.y - knot.y) {
                // straight up
                (0, 2) => {
                    *knot = knot.move_to(Direction::Up);
                }
                //straight right
                (2, 0) => {
                    *knot = knot.move_to(Direction::Right);
                }
                // straight down
                (0, -2) => {
                    *knot = knot.move_to(Direction::Down);
                }
                // straight left
                (-2, 0) => {
                    *knot = knot.move_to(Direction::Left);
                }
                // up and right
                (1, 2) | (2, 1) | (2, 2) => {
                    *knot = knot.move_to(Direction::Up).move_to(Direction::Right);
                }
                // down and right
                (1, -2) | (2, -1) | (2, -2) => {
                    *knot = knot.move_to(Direction::Down).move_to(Direction::Right);
                }
                // down and left
                (-2, -1) | (-1, -2) | (-2, -2) => {
                    *knot = knot.move_to(Direction::Down).move_to(Direction::Left);
                }
                // up and left
                (-2, 1) | (-1, 2) | (-2, 2) => {
                    *knot = knot.move_to(Direction::Up).move_to(Direction::Left);
                }
                _ => {
                    break;
                }
            }

            previous = *knot;
        }

        // record new tail position
        self.visited.insert(*knots.last().unwrap());
    }
}

struct Move {
    direction: Direction,
    steps: u8,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map_opt(anychar, |c| match c {
        'U' => Some(Direction::Up),
        'R' => Some(Direction::Right),
        'D' => Some(Direction::Down),
        'L' => Some(Direction::Left),
        _ => None,
    })(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, (direction, steps)) = separated_pair(parse_direction, tag(" "), u8)(input)?;
    Ok((input, Move { direction, steps }))
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| parse_move(line).unwrap().1)
        .collect::<Vec<_>>()
}

fn count_visited(knots: usize, input: &str) -> usize {
    let moves = parse_input(input);
    let mut rope = Rope::new(knots);

    for Move { direction, steps } in moves {
        for _ in 0..steps {
            rope.pull(direction)
        }
    }

    rope.visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1_with_two_knots() {
        let visited = count_visited(2, include_str!("sample-input-1.txt"));
        assert_eq!(visited, visited);
    }

    #[test]
    fn test_sample_input_part_1_with_ten_knots() {
        let visited = count_visited(10, include_str!("sample-input-1.txt"));
        assert_eq!(visited, visited);
    }

    #[test]
    fn test_sample_input_part_2() {
        let visited = count_visited(10, include_str!("sample-input-2.txt"));
        assert_eq!(36, visited);
    }
}

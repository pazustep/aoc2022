use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Context};
use std::fmt::Write;

fn main() {
    let input = include_str!("input.txt");
    let signal_strength = measure_signal_strength(input);
    dbg!(signal_strength);

    let crt = crt_to_string(draw_crt(input));
    print!("{crt}");
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');

        match iter.next() {
            Some("noop") => Ok(Self::Noop),
            Some("addx") => match iter.next() {
                Some(value) => value
                    .parse::<i32>()
                    .map(Self::Addx)
                    .context(format!("failed to parse {value} as i32")),
                None => Err(anyhow!("missing argument to addx")),
            },
            _ => Err(anyhow!(format!("expected one of noop,addx"))),
        }
    }
}

struct ExecutingInstruction(Instruction, u8);

impl From<Instruction> for ExecutingInstruction {
    fn from(instruction: Instruction) -> Self {
        match instruction {
            Instruction::Noop => Self(instruction, 1),
            Instruction::Addx(_) => Self(instruction, 2),
        }
    }
}

struct Cpu {
    queue: VecDeque<ExecutingInstruction>,
    cycle: u32,
    register_x: i32,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        let queue = instructions.into_iter().map(|i| i.into()).collect();

        Self {
            queue,
            cycle: 0,
            register_x: 1,
        }
    }
}

impl Iterator for Cpu {
    type Item = (u32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let register_x = self.register_x;

        match self.queue.pop_front() {
            None => return None,
            Some(ExecutingInstruction(Instruction::Noop, _)) => {}
            Some(ExecutingInstruction(Instruction::Addx(value), 2)) => {
                self.queue
                    .push_front(ExecutingInstruction(Instruction::Addx(value), 1));
            }
            Some(ExecutingInstruction(Instruction::Addx(value), 1)) => {
                self.register_x += value;
            }
            _ => panic!("invalid state"),
        }

        self.cycle += 1;

        // return value of register x _during_ the cycle, not after it
        Some((self.cycle, register_x))
    }
}

fn parse_instructions(input: &str) -> Cpu {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    Cpu::new(instructions)
}

fn measure_signal_strength(input: &str) -> i32 {
    parse_instructions(input).fold(0, |total, (cycle, x)| {
        if cycle == 20 || (cycle >= 60 && (cycle - 20) % 40 == 0) {
            let increase = x * cycle as i32;
            total + increase
        } else {
            total
        }
    })
}

fn draw_crt(input: &str) -> Vec<bool> {
    parse_instructions(input)
        .map(|(cycle, x)| {
            let sprite = (x - 1)..=(x + 1);
            let crt_pos = ((cycle - 1) % 40) as i32;

            sprite.contains(&crt_pos)
        })
        .collect()
}

fn crt_to_string(crt: Vec<bool>) -> String {
    let mut output = String::new();
    for (idx, pixel) in crt.iter().enumerate() {
        if *pixel {
            write!(output, "#").unwrap();
        } else {
            write!(output, ".").unwrap();
        }

        if (idx + 1) % 40 == 0 {
            writeln!(output).unwrap();
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_noop() {
        if let Ok(Instruction::Noop) = "noop".parse::<Instruction>() {
            // ok
        } else {
            panic!("should've matched a noop instruction")
        }
    }

    #[test]
    fn parse_addx_ok() {
        if let Ok(Instruction::Addx(-11)) = "addx -11".parse::<Instruction>() {
            // ok
        } else {
            panic!("should've matched an addx instruction")
        }
    }

    #[test]
    fn parse_addx_missing_argument() {
        match "addx".parse::<Instruction>() {
            Err(err) => assert_eq!(err.to_string(), "missing argument to addx"),
            _ => panic!("parse should've failed"),
        }
    }

    #[test]
    fn parse_unknown_command() {
        match "unknown".parse::<Instruction>() {
            Err(err) => assert_eq!(err.to_string(), "expected one of noop,addx"),
            _ => panic!("parse should've failed"),
        }
    }

    #[test]
    fn signal_strength_for_sample_input() {
        let signal_strength = measure_signal_strength(include_str!("sample-input.txt"));
        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn print_crt_with_sample_input() {
        let crt = draw_crt(include_str!("sample-input.txt"));
        let crt = crt_to_string(crt);
        assert_eq!(
            &crt,
            concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n"
            )
        )
    }
}

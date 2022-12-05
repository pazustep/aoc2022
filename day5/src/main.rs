use nom::bytes::complete::{is_a, tag, take, take_until};
use nom::character::complete::{anychar, u8};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{Finish, IResult};
use std::fmt::Formatter;

fn main() {
    let input = include_str!("input.txt");
    let (_, input) = parse_input(input).finish().unwrap();
    let mut dock = input.dock;

    for command in input.commands {
        dock.apply(command);
    }

    let tops = dock
        .stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().0)
        .collect::<String>();

    println!("Top of the stack is {tops}");
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct SupplyCrate(char);

impl std::fmt::Display for SupplyCrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Debug)]
struct LoadingDock {
    stacks: Vec<Vec<SupplyCrate>>,
}

impl LoadingDock {
    #[allow(clippy::expect_fun_call)]
    fn apply(&mut self, command: Command) {
        let from = self
            .stacks
            .get_mut(command.from - 1)
            .expect(&format!("can't find from stack {}", command.from));

        let start = from.len() - command.count;
        let mut elems = from.drain(start..).collect::<Vec<_>>();

        let to = self
            .stacks
            .get_mut(command.to - 1)
            .expect(&format!("can't find to stack {}", command.to));

        to.append(&mut elems);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input {
    dock: LoadingDock,
    commands: Vec<Command>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, dock) = parse_loading_dock(input)?;
    let (input, _) = take_until("move")(input)?;
    let (input, commands) = separated_list0(is_a("\r\n"), parse_command)(input)?;
    Ok((input, Input { dock, commands }))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = u8(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = u8(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = u8(input)?;

    Ok((
        input,
        Command {
            count: count as usize,
            from: from as usize,
            to: to as usize,
        },
    ))
}

fn parse_loading_dock(input: &str) -> IResult<&str, LoadingDock> {
    let (input, lines) = separated_list0(is_a("\r\n"), parse_loading_dock_line)(input)?;

    if lines.is_empty() {
        Ok((input, LoadingDock { stacks: vec![] }))
    } else {
        let line_size = if let Some(line) = lines.get(0) {
            line.len()
        } else {
            0
        };

        let mut stacks: Vec<Vec<SupplyCrate>> = (0..line_size).map(|_| Vec::new()).collect();

        for line in lines.into_iter().rev() {
            for (index, supply_crate) in line.into_iter().enumerate() {
                match (stacks.get_mut(index), supply_crate) {
                    (_, None) => {}
                    (Some(vec), Some(supply_crate)) => vec.push(supply_crate),
                    (None, Some(supply_crate)) => stacks.insert(index, vec![supply_crate]),
                }
            }
        }

        Ok((input, LoadingDock { stacks }))
    }
}

fn parse_loading_dock_line(input: &str) -> IResult<&str, Vec<Option<SupplyCrate>>> {
    let (input, list) = separated_list0(tag(" "), parse_supply_crate)(input)?;
    Ok((input, list))
}

fn parse_supply_crate(input: &str) -> IResult<&str, Option<SupplyCrate>> {
    let (input, chars) = take(3_usize)(input)?;

    if chars == "   " {
        Ok((input, None))
    } else {
        let (_, char) = delimited(tag("["), anychar, tag("]"))(chars)?;
        Ok((input, Some(SupplyCrate(char))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn test_parse_supply_crate_empty() {
        let (_, supply_crate) = parse_supply_crate("   ").finish().unwrap();
        assert!(supply_crate.is_none());
    }

    #[test]
    fn test_parse_supply_crate_complete() {
        let (remaining, supply_crate) = parse_supply_crate("[A]").finish().unwrap();
        assert!(remaining.is_empty());

        let supply_crate = supply_crate.unwrap();
        assert_eq!(supply_crate.0, 'A');
    }

    #[test]
    fn test_parse_supply_crate_partial() {
        let (remaining, supply_crate) = parse_supply_crate("[A]XX").finish().unwrap();
        assert_eq!(remaining, "XX");
        assert_eq!(supply_crate, Some(SupplyCrate('A')));
    }

    #[test]
    fn test_parse_loading_dock_line_one_crate() {
        let (remaining, line) = parse_loading_dock_line("[A]\n").finish().unwrap();
        assert_eq!(remaining, "\n");
        assert_eq!(line, vec![Some(SupplyCrate('A'))]);
    }

    #[test]
    fn test_parse_loading_dock_line_many_crates() {
        let (remaining, line) = parse_loading_dock_line("[A] [B] [C]").finish().unwrap();
        assert!(remaining.is_empty());
        assert_eq!(
            line,
            vec![
                Some(SupplyCrate('A')),
                Some(SupplyCrate('B')),
                Some(SupplyCrate('C'))
            ]
        );
    }

    #[test]
    fn test_parse_loading_dock_line_with_missing_crates() {
        let (remaining, line) = parse_loading_dock_line("    [B]    ").finish().unwrap();
        assert!(remaining.is_empty());
        assert_eq!(line, vec![None, Some(SupplyCrate('B')), None]);
    }

    #[test]
    fn test_parse_loading_dock() {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3"
        );

        let (remaining, loading_dock) = parse_loading_dock(input).finish().unwrap();
        assert_eq!(remaining, " 1   2   3");
        assert_eq!(
            loading_dock.stacks,
            vec![
                vec![SupplyCrate('Z'), SupplyCrate('N')],
                vec![SupplyCrate('M'), SupplyCrate('C'), SupplyCrate('D')],
                vec![SupplyCrate('P')],
            ]
        );
    }

    #[test]
    fn test_parse_command() {
        let (remaining, command) = parse_command("move 1 from 2 to 3\n").finish().unwrap();
        assert_eq!(remaining, "\n");
        assert_eq!(
            command,
            Command {
                count: 1,
                from: 2,
                to: 3
            }
        );
    }

    #[test]
    fn test_parse_input() {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2\n"
        );

        let (remaining, input) = parse_input(input).finish().unwrap();
        assert_eq!(remaining, "\n");
        assert_eq!(input.dock.stacks.len(), 3);
        assert_eq!(input.commands.len(), 4);
    }
}

use std::vec;

use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn main() {
    let mut lines = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = FsEntry::root().build(&mut lines);
    dbg!(&root);

    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();

    dbg!(sum);

    let total_space = 70_000_000_u64;
    let used_space = root.total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30_000_000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let removed_dir_size = root
        .children
        .iter()
        .filter(|&n| !n.children.is_empty())
        .map(|n| n.total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();

    dbg!(removed_dir_size);
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
struct FsEntry {
    size: u64,
    #[allow(dead_code)]
    path: Utf8PathBuf,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn root() -> Self {
        Self {
            size: 0,
            path: "/".into(),
            children: vec![],
        }
    }

    fn new(path: &Utf8PathBuf) -> Self {
        Self::new_with_size(0, path)
    }

    fn new_with_size(size: u64, path: &Utf8PathBuf) -> Self {
        Self {
            size,
            path: path.clone(),
            children: vec![],
        }
    }

    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }

    fn build(mut self, it: &mut dyn Iterator<Item = Line>) -> Self {
        while let Some(line) = it.next() {
            match line {
                Line::Command(Command::Cd(path)) => match path.as_str() {
                    "/" => {
                        // ignore
                    }
                    ".." => break,
                    _ => {
                        let node = FsEntry::new(&path);
                        self.children.push(node.build(it));
                    }
                },
                Line::Entry(Entry::File(size, path)) => {
                    let node = FsEntry::new_with_size(size, &path);
                    self.children.push(node);
                }
                _ => {
                    // ignore
                }
            }
        }

        self
    }
}

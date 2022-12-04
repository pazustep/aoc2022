use im::HashSet;
use item::Item;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| b.try_into().unwrap())
                .collect::<HashSet<Item>>()
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            chunks
                .reduce(|a, b| a.intersection(b))
                .expect("we always have 3 chunks")
                .iter()
                .next()
                .expect("problem statement says there is always one item in common")
                .priority()
        })
        .sum();

    dbg!(sum);
    Ok(())
}

mod item {
    use color_eyre::{eyre::eyre, Report};

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl Item {
        pub(crate) fn priority(self) -> usize {
            match self {
                Self(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Self(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl TryFrom<u8> for Item {
        type Error = Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(eyre!("{} is not a valid item", value as char)),
            }
        }
    }
}

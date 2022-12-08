fn main() {
    let input = include_str!("input.txt");
    let forest = Forest::parse(input);

    let visible_count = forest
        .trees()
        .filter(|tree| forest.is_visible(tree))
        .count();

    println!("number of visible trees: {visible_count}");
}

struct Tree {
    x: usize,
    y: usize,
    height: u8,
}

struct Forest {
    trees: Vec<Vec<u8>>,
}

impl Forest {
    fn parse(input: &str) -> Self {
        let trees = input
            .lines()
            .map(|line| line.bytes().map(|byte| byte - b'0').collect())
            .collect();

        Self { trees }
    }

    fn trees(&self) -> impl Iterator<Item = Tree> + '_ {
        self.trees.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, height)| Tree {
                x,
                y,
                height: *height,
            })
        })
    }

    fn is_visible(&self, tree: &Tree) -> bool {
        self.is_visible_from_left(tree)
            || self.is_visible_from_right(tree)
            || self.is_visible_from_top(tree)
            || self.is_visible_from_bottom(tree)
    }

    fn is_visible_from_top(&self, tree: &Tree) -> bool {
        for iy in 0..tree.y {
            let height = self.trees.get(iy).unwrap().get(tree.x).unwrap();
            if *height >= tree.height {
                return false;
            }
        }

        true
    }

    fn is_visible_from_bottom(&self, tree: &Tree) -> bool {
        for iy in (tree.y + 1)..(self.trees.len()) {
            let height = self.trees.get(iy).unwrap().get(tree.x).unwrap();
            if *height >= tree.height {
                return false;
            }
        }

        true
    }

    fn is_visible_from_left(&self, tree: &Tree) -> bool {
        let row = self.trees.get(tree.y).unwrap();

        for ix in 0..tree.x {
            let height = row.get(ix).unwrap();
            if *height >= tree.height {
                return false;
            }
        }

        true
    }

    fn is_visible_from_right(&self, tree: &Tree) -> bool {
        let row = self.trees.get(tree.y).unwrap();

        for ix in (tree.x + 1)..row.len() {
            let height = row.get(ix).unwrap();
            if *height >= tree.height {
                return false;
            }
        }

        true
    }
}

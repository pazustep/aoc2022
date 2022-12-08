fn main() {
    let forest = Forest::parse(include_str!("input.txt"));
    println!("number of visible trees: {}", forest.visible_tree_count());
    println!("best scenic score: {}", forest.best_scenic_score().unwrap());
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

    fn visible_tree_count(&self) -> usize {
        self.trees().filter(|tree| self.is_visible(tree)).count()
    }

    fn best_scenic_score(&self) -> Option<u32> {
        self.trees().map(|tree| self.scenic_score(&tree)).max()
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

    fn scenic_score(&self, tree: &Tree) -> u32 {
        self.scenic_score_top(tree)
            * self.scenic_score_left(tree)
            * self.scenic_score_right(tree)
            * self.scenic_score_bottom(tree)
    }

    fn scenic_score_top(&self, tree: &Tree) -> u32 {
        let mut score = 0;

        for iy in (0..tree.y).rev() {
            score += 1;
            if *self.trees.get(iy).unwrap().get(tree.x).unwrap() >= tree.height {
                break;
            }
        }

        score
    }

    fn scenic_score_bottom(&self, tree: &Tree) -> u32 {
        let mut score = 0;

        for iy in (tree.y + 1)..(self.trees.len()) {
            score += 1;
            if *self.trees.get(iy).unwrap().get(tree.x).unwrap() >= tree.height {
                break;
            }
        }

        score
    }

    fn scenic_score_left(&self, tree: &Tree) -> u32 {
        let mut score = 0;
        let row = self.trees.get(tree.y).unwrap();

        for ix in (0..tree.x).rev() {
            score += 1;
            if *row.get(ix).unwrap() >= tree.height {
                break;
            }
        }

        score
    }

    fn scenic_score_right(&self, tree: &Tree) -> u32 {
        let mut score = 0;
        let row = self.trees.get(tree.y).unwrap();

        for ix in (tree.x + 1)..row.len() {
            score += 1;
            if *row.get(ix).unwrap() >= tree.height {
                break;
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenic_score_first_example() {
        let forest = Forest::parse(include_str!("sample-input.txt"));

        let tree = Tree {
            y: 1,
            x: 2,
            height: 5,
        };

        assert_eq!(forest.scenic_score_top(&tree), 1);
        assert_eq!(forest.scenic_score_left(&tree), 1);
        assert_eq!(forest.scenic_score_right(&tree), 2);
        assert_eq!(forest.scenic_score_bottom(&tree), 2);
        assert_eq!(forest.scenic_score(&tree), 4);
    }

    #[test]
    fn visible_tree_count() {
        let forest = Forest::parse(include_str!("sample-input.txt"));
        assert_eq!(forest.visible_tree_count(), 21);
    }

    #[test]
    fn best_scenic_score() {
        let forest = Forest::parse(include_str!("sample-input.txt"));
        assert_eq!(forest.best_scenic_score(), Some(8));
    }
}

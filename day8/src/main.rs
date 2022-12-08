fn main() {
    let input = include_str!("input.txt");
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect();

    let mut total_count = 0;
    for y in 0..grid.len() {
        let row = grid.get(y).unwrap();

        for x in 0..row.len() {
            if is_visible(&grid, x, y) {
                total_count += 1;
            }
        }
    }

    println!("number of visible trees: {total_count}");
}

fn is_visible(grid: &[Vec<u8>], x: usize, y: usize) -> bool {
    let subject_height = *grid.get(y).unwrap().get(x).unwrap();
    is_visible_from_left(grid, x, y, subject_height)
        || is_visible_from_right(grid, x, y, subject_height)
        || is_visible_from_top(grid, x, y, subject_height)
        || is_visible_from_bottom(grid, x, y, subject_height)
}

fn is_visible_from_top(grid: &[Vec<u8>], x: usize, y: usize, subject_height: u8) -> bool {
    for iy in 0..y {
        let height = grid.get(iy).unwrap().get(x).unwrap();
        if *height >= subject_height {
            return false;
        }
    }

    true
}

fn is_visible_from_bottom(grid: &[Vec<u8>], x: usize, y: usize, subject_height: u8) -> bool {
    for iy in (y + 1)..grid.len() {
        let height = grid.get(iy).unwrap().get(x).unwrap();
        if *height >= subject_height {
            return false;
        }
    }

    true
}

fn is_visible_from_left(grid: &[Vec<u8>], x: usize, y: usize, subject_height: u8) -> bool {
    let row = grid.get(y).unwrap();

    for ix in 0..x {
        let height = row.get(ix).unwrap();
        if *height >= subject_height {
            return false;
        }
    }

    true
}

fn is_visible_from_right(grid: &[Vec<u8>], x: usize, y: usize, subject_height: u8) -> bool {
    let row = grid.get(y).unwrap();

    for ix in (x + 1)..row.len() {
        let height = row.get(ix).unwrap();
        if *height >= subject_height {
            return false;
        }
    }

    true
}

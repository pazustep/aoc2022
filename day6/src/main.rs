use itertools::Itertools;

const WINDOW_SIZE: usize = 4;

fn main() {
    let input = include_str!("input.txt");
    let idx = find_start_of_packet(input);
    println!("start of packet: {idx:?}");
}

fn find_start_of_packet(input: &str) -> Option<usize> {
    input.as_bytes()
        .windows(WINDOW_SIZE)
        .position(|slice| slice.iter().all_unique())
        .map(|idx| idx + WINDOW_SIZE)
}

#[cfg(test)]
mod tests {
    use crate::find_start_of_packet;

    #[test]
    fn test_find_start_of_packet_with_sample_input() {
        let idx = find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(idx, Some(7));
    }
}

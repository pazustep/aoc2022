use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let start = find_start_of_packet(input);
    println!("start of packet: {start:?}");
}

fn find_start_of_packet(input: &str) -> Option<usize> {
    for (idx, window) in input.as_bytes().windows(4).enumerate() {
        if window.iter().all_unique() {
            return Some(idx + 4);
        }
    }

    None
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

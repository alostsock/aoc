use crate::Solution;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day6 {}

impl Solution for Day6 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        first_marker(include_str!("data/day06"), 4)
    }

    fn part_2(&self) -> Self::P2 {
        first_marker(include_str!("data/day06"), 14)
    }
}

fn first_marker(s: &str, length: usize) -> usize {
    let slice: Vec<_> = s.chars().collect();
    for (i, window) in slice.windows(length).enumerate() {
        let set: HashSet<&char> = window.iter().collect();
        if set.len() == length {
            return i + length;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_marker_works() {
        assert_eq!(first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn first_marker_long_works() {
        assert_eq!(first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(first_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}

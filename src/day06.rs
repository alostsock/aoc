use crate::Solution;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day6 {}

impl Solution for Day6 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        find_fast(include_str!("data/day06"), 4)
    }

    fn part_2(&self) -> Self::P2 {
        find_fast(include_str!("data/day06"), 14)
    }
}

#[allow(dead_code)]
fn find(s: &str, length: usize) -> usize {
    let slice: Vec<_> = s.chars().collect();
    let index = slice
        .windows(length)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == length)
        .unwrap();
    index + length
}

#[derive(Default)]
struct Lookup {
    unique_char_count: usize,
    char_counts: [usize; 26],
}

impl Lookup {
    fn code(char: char) -> usize {
        (char as usize) - ('a' as usize)
    }

    fn add(&mut self, char: char) {
        let i = Self::code(char);

        self.char_counts[i] += 1;
        if self.char_counts[i] == 1 {
            self.unique_char_count += 1;
        }
    }

    fn sub(&mut self, char: char) {
        let i = Self::code(char);

        self.char_counts[i] -= 1;
        if self.char_counts[i] == 0 {
            self.unique_char_count -= 1;
        }
    }
}

fn find_fast(s: &str, length: usize) -> usize {
    let mut lookup = Lookup::default();

    // initialize `seen`
    for char in s.get(0..length - 1).unwrap().chars() {
        lookup.add(char);
    }

    let slice: Vec<_> = s.chars().collect();
    for (i, window) in slice.windows(length).enumerate() {
        lookup.add(*window.last().unwrap());

        if lookup.unique_char_count == length {
            return i + length;
        }

        lookup.sub(*window.first().unwrap());
    }

    panic!("didn't find a marker")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_works() {
        assert_eq!(find("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        assert_eq!(find("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn find_fast_works() {
        assert_eq!(find_fast("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find_fast("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_fast("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_fast("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_fast("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        assert_eq!(find_fast("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_fast("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_fast("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_fast("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_fast("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}

use crate::Solution;
use std::collections::{HashMap, HashSet};

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

fn find_fast(s: &str, length: usize) -> usize {
    let mut unique_char_count = 0;
    let mut seen = HashMap::<char, usize>::new();

    // initialize `seen`
    for char in s.get(0..length - 1).unwrap().chars() {
        seen.entry(char)
            .and_modify(|count| {
                *count += 1;
                if *count == 1 {
                    unique_char_count += 1;
                }
            })
            .or_insert_with(|| {
                unique_char_count += 1;
                1
            });
    }

    // iterate through the rest
    let slice: Vec<_> = s.chars().collect();
    for (i, window) in slice.windows(length).enumerate() {
        // add last char
        seen.entry(*window.last().unwrap())
            .and_modify(|count| {
                *count += 1;
                if *count == 1 {
                    unique_char_count += 1;
                }
            })
            .or_insert_with(|| {
                unique_char_count += 1;
                1
            });

        if unique_char_count == length {
            return i + length;
        }

        // remove first char
        seen.entry(*window.first().unwrap()).and_modify(|count| {
            *count -= 1;
            if *count == 0 {
                unique_char_count -= 1;
            }
        });
    }

    0
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

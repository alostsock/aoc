use crate::Solution;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day3 {}

impl Solution for Day3 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        include_str!("data/day3")
            .lines()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                let a: HashSet<char> = a.chars().collect();
                let b: HashSet<char> = b.chars().collect();
                let unique_item = a.intersection(&b).next().unwrap();
                priority_from_item(*unique_item)
            })
            .sum()
    }

    fn part_2(&self) -> Self::Result {
        // `Iterator::array_chunks` would be perfect for this, but it's not in stable yet
        // https://github.com/rust-lang/rust/issues/100450
        include_str!("data/day3")
            .lines()
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|rucksacks| {
                let a: HashSet<char> = rucksacks[0].chars().collect();
                let b: HashSet<char> = rucksacks[1].chars().collect();
                let ab: HashSet<&char> = a.intersection(&b).collect();
                let unique_item = rucksacks[2].chars().find(|item| ab.contains(item)).unwrap();
                priority_from_item(unique_item)
            })
            .sum()
    }
}

fn priority_from_item(item: char) -> usize {
    match item {
        'a'..='z' => (item as usize) - ('a' as usize) + 1,
        'A'..='Z' => (item as usize) - ('A' as usize) + 27,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_works() {
        assert_eq!(priority_from_item('a'), 1);
        assert_eq!(priority_from_item('z'), 26);
        assert_eq!(priority_from_item('A'), 27);
        assert_eq!(priority_from_item('Z'), 52);
    }
}
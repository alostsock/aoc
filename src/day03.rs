use crate::{utils::read_lines, Solution};
use std::collections::HashSet;

#[derive(Default)]
pub struct Day3 {}

impl Solution for Day3 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        read_lines("src/data/day03")
            .unwrap()
            .flatten()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                let a: HashSet<char> = a.chars().collect();
                let b: HashSet<char> = b.chars().collect();
                let unique_item = a.intersection(&b).next().unwrap();
                priority_from_item(*unique_item)
            })
            .sum()
    }

    fn part_2(&self) -> Self::P2 {
        // `Iterator::array_chunks` would be perfect for this, but it's not in stable yet
        // https://github.com/rust-lang/rust/issues/100450
        read_lines("src/data/day03")
            .unwrap()
            .flatten()
            .collect::<Vec<String>>()
            .chunks_exact(3)
            .map(|rucksacks| {
                let a: HashSet<char> = rucksacks[0].chars().collect();
                let b: HashSet<char> = rucksacks[1].chars().collect();
                let ab: HashSet<&char> = a.intersection(&b).collect();
                let unique_item = rucksacks[2]
                    .chars()
                    .find(|&item| ab.contains(&item))
                    .unwrap();
                priority_from_item(unique_item)
            })
            .sum()
    }
}

fn priority_from_item(item: char) -> usize {
    let code = item as usize;
    match code {
        97..=122 => code - 96,
        65..=90 => code - 38,
        _ => 0,
    }
}

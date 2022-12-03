use crate::Solution;

#[derive(Default)]
pub struct Day1 {}

impl Solution for Day1 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        let mut max_calories = 0;
        let mut current_calories = 0;
        for line in include_str!("data/day01").lines() {
            if let Ok(calories) = line.parse::<usize>() {
                current_calories += calories;
            } else {
                max_calories = max_calories.max(current_calories);
                current_calories = 0;
            }
        }
        max_calories
    }

    fn part_2(&self) -> Self::P2 {
        let mut elves: Vec<usize> = vec![0];
        for line in include_str!("data/day01").lines() {
            if let Ok(calories) = line.parse::<usize>() {
                *elves.last_mut().unwrap() += calories;
            } else {
                elves.push(0);
            }
        }
        elves.sort_unstable();
        elves.reverse();
        elves.iter().take(3).sum()
    }
}

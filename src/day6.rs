use crate::Solution;

#[derive(Default)]
pub struct Day6 {}

impl Solution for Day6 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day6");
        count_options_v1(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day6");
        count_options_v2(input)
    }
}

fn count_race_options(time: usize, distance: usize) -> usize {
    let mut options = 0;
    for t in 1..time {
        let d = t * (time - t);
        if d > distance {
            options += 1;
        }
    }
    options
}

fn count_options_v1(input: &str) -> usize {
    let (times_line, distances_line) = input.trim().split_once("\n").unwrap();
    let (_, times_str) = times_line.split_once(':').unwrap();
    let (_, distances_str) = distances_line.split_once(':').unwrap();
    let times = times_str
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap());
    let distances = distances_str
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());

    let races = times.zip(distances);
    let mut product = 1;

    for (time, distance) in races {
        let options = count_race_options(time, distance);
        product *= options;
    }

    product
}

fn count_options_v2(input: &str) -> usize {
    let (times_line, distances_line) = input.trim().split_once("\n").unwrap();
    let (_, times_str) = times_line.split_once(':').unwrap();
    let (_, distances_str) = distances_line.split_once(':').unwrap();
    let time = times_str.replace(" ", "").parse().unwrap();
    let distance = distances_str.replace(" ", "").parse().unwrap();

    count_race_options(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        assert_eq!(count_options_v1(INPUT), 288);
        assert_eq!(Day6::new().part_1(), 3316275);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_options_v2(INPUT), 71503);
        assert_eq!(Day6::new().part_2(), 27102791);
    }
}

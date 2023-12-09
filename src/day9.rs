use crate::Solution;

#[derive(Default)]
pub struct Day9 {}

impl Solution for Day9 {
    type Result = i64;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day9");
        sum_next_values(input, false)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day9");
        sum_next_values(input, true)
    }
}

fn sum_next_values(input: &str, reverse: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut sequence: Vec<_> = line
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            if reverse {
                sequence.reverse();
            }
            extrapolate_next_value(sequence)
        })
        .sum()
}

fn extrapolate_next_value(sequence: Vec<i64>) -> i64 {
    if sequence.iter().all(|&n| n == 0) {
        return 0;
    }

    let next_sequence = sequence
        .iter()
        .zip(sequence.iter().skip(1))
        .map(|(n0, n1)| n1 - n0)
        .collect();

    sequence.last().unwrap() + extrapolate_next_value(next_sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1() {
        assert_eq!(sum_next_values(INPUT, false), 114);
        assert_eq!(Day9::new().part_1(), 2075724761);
    }

    #[test]
    fn part_2() {
        assert_eq!(sum_next_values(INPUT, true), 2);
        assert_eq!(Day9::new().part_2(), 1072);
    }
}

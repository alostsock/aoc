use crate::Solution;

#[derive(Default)]
pub struct Day1 {}

impl Solution for Day1 {
    type Result = isize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day1");
        times_at_zero(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day1");
        times_passed_zero(input)
    }
}

fn times_at_zero(input: &str) -> isize {
    let mut position = 50;
    let mut times = 0;

    for line in input.lines() {
        let (direction, num_str) = line.split_at(1);
        let num: isize = num_str.parse().unwrap();
        if direction == "L" {
            position -= num % 100;
        } else {
            position += num % 100;
        }

        if position < 0 {
            position += 100;
        } else if position > 99 {
            position -= 100;
        }

        if position == 0 {
            times += 1;
        }
    }

    times
}

fn times_passed_zero(input: &str) -> isize {
    let mut position = 50;
    let mut times = 0;

    for line in input.lines() {
        let (direction, num_str) = line.split_at(1);
        let num: isize = num_str.parse().unwrap();

        let start_position = position;

        if direction == "L" {
            times += num / 100;
            position -= num % 100;
        } else {
            times += num / 100;
            position += num % 100;
        }

        if position < 0 {
            position += 100;
            if position != 0 && start_position != 0 {
                times += 1;
            }
        } else if position > 99 {
            position -= 100;
            if position != 0 && start_position != 0 {
                times += 1;
            }
        }

        if position == 0 {
            times += 1;
        }
    }

    times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(times_at_zero(input), 3);
    }

    #[test]
    fn part_2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(times_passed_zero(input), 6);
    }
}

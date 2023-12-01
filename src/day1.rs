use crate::Solution;

#[derive(Default)]
pub struct Day1 {}

impl Solution for Day1 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day1");
        calibration_value(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day1");
        calibration_value_lettered(input)
    }
}

fn calibration_value(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        let line_value = (first_digit * 10 + last_digit) as usize;
        sum += line_value;
    }
    sum
}

const LETTERED_DIGITS: [(usize, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn calibration_value_lettered(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit_position = line.len();
        let mut first_digit = 0;
        for (digit, lettered_digit) in LETTERED_DIGITS {
            if let Some(position) = line.find(lettered_digit) {
                if position < first_digit_position {
                    first_digit_position = position;
                    first_digit = digit;
                }
            }
        }
        for (position, char) in line.chars().enumerate() {
            if char.is_numeric() && position < first_digit_position {
                first_digit_position = position;
                first_digit = char.to_digit(10).unwrap() as usize;
            }
        }

        let mut last_digit_position = 0;
        let mut last_digit = first_digit;
        for (digit, lettered_digit) in LETTERED_DIGITS {
            if let Some(position) = line.rfind(lettered_digit) {
                if position > last_digit_position {
                    last_digit_position = position;
                    last_digit = digit;
                }
            }
        }
        for (index, char) in line.chars().rev().enumerate() {
            let position = line.len() - 1 - index;
            if char.is_numeric() && position > last_digit_position {
                last_digit_position = position;
                last_digit = char.to_digit(10).unwrap() as usize;
            }
        }

        let line_value = first_digit * 10 + last_digit;
        sum += line_value;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(calibration_value(input), 142);
    }

    #[test]
    fn part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(calibration_value_lettered(input), 281);
    }
}

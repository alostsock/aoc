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
        calibration_values_lettered(input)
    }
}

fn calibration_value(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let line_value: usize = format!("{first_digit}{last_digit}").parse().unwrap();
        sum += line_value;
    }
    sum
}

const LETTERED_DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn calibration_values_lettered(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit_position = line.len();
        let mut first_digit = 0;
        for (index, lettered_digit) in LETTERED_DIGITS.iter().enumerate() {
            let digit = index + 1;
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
        for (index, lettered_digit) in LETTERED_DIGITS.iter().enumerate() {
            let digit = index + 1;
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

        let line_value: usize = format!("{first_digit}{last_digit}").parse().unwrap();
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

        assert_eq!(calibration_values_lettered(input), 281);
    }
}

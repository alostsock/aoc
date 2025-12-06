use crate::Solution;

#[derive(Default)]
pub struct Day3 {}

impl Solution for Day3 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day3");
        maximum_joltage(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day3");
        maximum_joltage_12(input)
    }
}

fn maximum_joltage(input: &str) -> usize {
    let mut max_joltage = 0;

    for bank in input.lines() {
        let mut bank_max = 0;
        let batteries: Vec<char> = bank.chars().collect();
        for (first_pos, &first_battery) in batteries.iter().enumerate() {
            for &second_battery in &batteries[first_pos + 1..batteries.len()] {
                let mut joltage = first_battery.to_string();
                joltage.push_str(&second_battery.to_string());
                bank_max = bank_max.max(joltage.parse().expect("valid joltage"));
            }
        }
        max_joltage += bank_max;
    }

    max_joltage
}

fn maximum_joltage_12(input: &str) -> usize {
    let mut max_joltage = 0;

    for bank in input.lines() {
        let mut switched_value = String::new();
        // 9876 54321111111 | len 15
        let mut remaining_batteries: Vec<char> = bank.chars().collect();
        for switch in 0..12 {
            let required_remaining = 11 - switch;

            let best_battery = &remaining_batteries
                [0..remaining_batteries.len() - required_remaining]
                .iter()
                .max()
                .unwrap();
            switched_value.push_str(&best_battery.to_string());
            let best_battery_position = remaining_batteries
                .iter()
                .position(|b| b == *best_battery)
                .unwrap();
            remaining_batteries = remaining_batteries[best_battery_position + 1..].to_vec();
        }

        max_joltage += switched_value.parse::<usize>().unwrap();
    }

    max_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(maximum_joltage(input), 357);
    }

    #[test]
    fn part_2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(maximum_joltage_12(input), 3121910778619);
    }
}

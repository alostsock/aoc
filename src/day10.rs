#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use crate::Solution;

#[derive(Default)]
pub struct Day10 {}

impl Solution for Day10 {
    type Result = i32;

    fn part_1(&self) -> Self::Result {
        let signals = signal_strengths(include_str!("data/day10"));
        signals.iter().sum()
    }

    fn part_2(&self) -> Self::Result {
        let screen = draw_screen(include_str!("data/day10"), ' ', '█');
        for line in screen {
            println!("{}", line.iter().collect::<String>());
        }
        0
    }
}

fn schedule_values(input: &str, cycles: usize) -> Vec<i32> {
    let ops = input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>());

    let mut scheduled_values: Vec<i32> = vec![0; cycles];
    let mut scheduled_index = 0;

    for op in ops {
        match &*op {
            ["noop"] => scheduled_index += 1,
            ["addx", value_str] => {
                scheduled_index += 2;
                scheduled_values[scheduled_index] += value_str.parse::<i32>().unwrap();
            }
            _ => panic!("invalid operation"),
        }
    }

    scheduled_values
}

fn signal_strengths(input: &str) -> Vec<i32> {
    const CYCLES: usize = 220 * 2;

    let scheduled_values = schedule_values(input, CYCLES);

    let mut signal_strengths = vec![];
    let mut signal_strength: i32 = 1;

    for (cycle, value) in (1..=CYCLES).zip(scheduled_values.iter()) {
        signal_strength += value;

        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            signal_strengths.push((cycle as i32) * signal_strength);
        }
    }

    signal_strengths
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn draw_screen(input: &str, empty: char, filled: char) -> [[char; WIDTH]; HEIGHT] {
    const CYCLES: usize = 240;

    let scheduled_values = schedule_values(input, CYCLES);

    let mut screen = [[empty; WIDTH]; HEIGHT];
    let mut sprite_position: i32 = 1;

    for (cycle, value) in (1..=CYCLES).zip(scheduled_values.iter()) {
        sprite_position += value;

        let y = (cycle as i32 - 1) / (WIDTH as i32);
        let x = (cycle as i32 - 1) % (WIDTH as i32);
        if (sprite_position - 1..=sprite_position + 1).contains(&x) {
            screen[y as usize][x as usize] = filled;
        }
    }

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("data/day10_test");

    #[test]
    fn signal_strengths_works() {
        let signals = signal_strengths(TEST_INPUT);

        assert_eq!(signals, vec![420, 1140, 1800, 2940, 2880, 3960]);
        assert_eq!(signals.iter().sum::<i32>(), 13140);
    }

    #[test]
    fn drawing_works() {
        let screen = draw_screen(TEST_INPUT, '.', '#');

        assert_eq!(
            screen
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<_>>(),
            vec![
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
        );
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day10::new().part_1(), 13680);
    }
}

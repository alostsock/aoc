#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate, clippy::must_use_unit)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod example;

use std::time::Instant;

pub fn run_solution(day: u8, part: Option<u8>) {
    match day {
        0 => example::Example::new().run(part),
        1 => day01::Day1::new().run(part),
        2 => day02::Day2::new().run(part),
        3 => day03::Day3::new().run(part),
        4 => day04::Day4::new().run(part),
        5 => day05::Day5::new().run(part),
        6 => day06::Day6::new().run(part),
        _ => panic!("day {} hasn't been implemented yet", day),
    };
}

macro_rules! time {
    ($e: expr) => {{
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();
        (result, duration)
    }};
}

pub trait Solution {
    type P1;
    type P2;

    fn part_1(&self) -> Self::P1;
    fn part_2(&self) -> Self::P2;

    fn new() -> Self
    where
        Self: std::default::Default,
    {
        Self::default()
    }

    fn run(&self, part: Option<u8>)
    where
        Self::P1: std::fmt::Display,
        Self::P2: std::fmt::Display,
    {
        if part.is_none() || part.unwrap() == 1 {
            let (result, duration) = time!(self.part_1());
            println!("Part 1 solution ({:?}): {}", duration, result);
        }
        if part.is_none() || part.unwrap() == 2 {
            let (result, duration) = time!(self.part_2());
            println!("Part 2 solution ({:?}): {}", duration, result);
        }
    }
}

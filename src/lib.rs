mod example;

use std::time::Instant;

pub fn run_solution(day: u8, part: Option<u8>) {
    match day {
        0 => example::Example::new().run(part),
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
    type Data;
    type P1;
    type P2;

    fn data(&self) -> Self::Data;
    fn part_1(&self, data: &Self::Data) -> Self::P1;
    fn part_2(&self, data: &Self::Data) -> Self::P2;

    fn new() -> Self
    where
        Self: std::default::Default,
    {
        Default::default()
    }

    fn run(&self, part: Option<u8>)
    where
        Self::P1: std::fmt::Display,
        Self::P2: std::fmt::Display,
    {
        let (data, duration) = time!(self.data());
        println!("Data loaded ({:?})", duration);

        if part.is_none() || part.unwrap() == 1 {
            let (result, duration) = time!(self.part_1(&data));
            println!("Part 1 solution ({:?}): {}", duration, result);
        }
        if part.is_none() || part.unwrap() == 2 {
            let (result, duration) = time!(self.part_2(&data));
            println!("Part 2 solution ({:?}): {}", duration, result);
        }
    }
}

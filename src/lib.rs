#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate, clippy::must_use_unit)]

use seq_macro::seq;
use std::time::Instant;

macro_rules! time {
    ($e: expr) => {{
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();
        (result, duration)
    }};
}

seq!(N in 1..=16 {
    mod example;
    #(mod day~N;)*

    pub fn solve(day: Option<u8>, part: Option<u8>) {
        let days = if let Some(day) = day {
            vec![day]
        } else {
            vec![#(N,)*]
        };

        let (_, duration) = time!(
            for day in days {
                println!("\nRunning solution for day {}...", day);
                match day {
                    0 => example::Example::new().run(part),
                    #(N => day~N::Day~N::new().run(part),)*
                    _ => (),
                };
            }
        );

        println!("\n{:?} elapsed.", duration);
    }
});

pub trait Solution {
    type Result;

    fn part_1(&self) -> Self::Result;
    fn part_2(&self) -> Self::Result;

    fn new() -> Self
    where
        Self: std::default::Default,
    {
        Self::default()
    }

    fn run(&self, part: Option<u8>)
    where
        Self::Result: std::fmt::Display,
    {
        if part.is_none() || part.unwrap() == 1 {
            let (result, duration) = time!(self.part_1());
            println!("Part 1 ({:?}): {}", duration, result);
        }

        if part.is_none() || part.unwrap() == 2 {
            let (result, duration) = time!(self.part_2());
            println!("Part 2 ({:?}): {}", duration, result);
        }
    }
}

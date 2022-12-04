use crate::Solution;

#[derive(Default)]
pub struct Day4 {}

impl Solution for Day4 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        include_str!("data/day04")
            .lines()
            .filter(|line| {
                let (a, b) = line.split_once(',').unwrap();
                let (a_start, a_end) = a.split_once('-').unwrap();
                let (b_start, b_end) = b.split_once('-').unwrap();
                is_overlapping(n(a_start), n(a_end), n(b_start), n(b_end))
            })
            .count()
    }

    fn part_2(&self) -> Self::P2 {
        include_str!("data/day04")
            .lines()
            .filter(|line| {
                let (a, b) = line.split_once(',').unwrap();
                let (a_start, a_end) = a.split_once('-').unwrap();
                let (b_start, b_end) = b.split_once('-').unwrap();
                is_overlapping_partial(n(a_start), n(a_end), n(b_start), n(b_end))
            })
            .count()
    }
}

fn n(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn is_overlapping(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    // b in a
    (a_start <= b_start && a_end >= b_end)
    // a in b
    || (b_start <= a_start && b_end >= a_end)
}

fn is_overlapping_partial(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    // a ahead of b
    (a_start <= b_start && a_end >= b_start)
    // b ahead of a
    || (b_start <= a_start && b_end >= a_start)
}

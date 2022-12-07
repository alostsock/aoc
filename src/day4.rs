use crate::Solution;

#[derive(Default)]
pub struct Day4 {}

impl Solution for Day4 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        include_str!("data/day4")
            .lines()
            .map(numbers)
            .filter(|(a, b, c, d)| is_overlapping(*a, *b, *c, *d))
            .count()
    }

    fn part_2(&self) -> Self::Result {
        include_str!("data/day4")
            .lines()
            .map(numbers)
            .filter(|(a, b, c, d)| is_overlapping_partial(*a, *b, *c, *d))
            .count()
    }
}

fn numbers(line: &str) -> (usize, usize, usize, usize) {
    let nums: Vec<_> = line
        .split(&['-', ','])
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    (nums[0], nums[1], nums[2], nums[3])
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

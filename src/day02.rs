use crate::Solution;

#[derive(Default)]
pub struct Day2 {}

impl Solution for Day2 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        let mut total_score = 0;
        for line in include_str!("data/day02").lines() {
            let (theirs, mine) = line.split_once(' ').unwrap();
            total_score += score(theirs, mine);
        }
        total_score
    }

    fn part_2(&self) -> Self::P2 {
        let mut total_score = 0;
        for line in include_str!("data/day02").lines() {
            let (theirs, end_state) = line.split_once(' ').unwrap();
            total_score += playout(theirs, end_state);
        }
        total_score
    }
}

fn score(theirs: &str, mine: &str) -> usize {
    match (theirs, mine) {
        // draw
        ("A", "X") => 4,
        ("B", "Y") => 5,
        ("C", "Z") => 6,
        // win
        ("C", "X") => 7,
        ("A", "Y") => 8,
        ("B", "Z") => 9,
        // loss
        (_, "X") => 1,
        (_, "Y") => 2,
        (_, "Z") => 3,
        // ???
        _ => 0,
    }
}

fn playout(theirs: &str, end_state: &str) -> usize {
    match (theirs, end_state) {
        // lose
        ("A", "X") => 3,
        ("B", "X") => 1,
        ("C", "X") => 2,
        // draw
        ("A", "Y") => 4,
        ("B", "Y") => 5,
        ("C", "Y") => 6,
        // win
        ("A", "Z") => 8,
        ("B", "Z") => 9,
        ("C", "Z") => 7,
        // ???
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring_works() {
        assert_eq!(score("A", "Y"), 8);
        assert_eq!(score("B", "X"), 1);
        assert_eq!(score("C", "Z"), 6);
    }

    #[test]
    fn playing_works() {
        assert_eq!(playout("A", "Y"), 4);
        assert_eq!(playout("B", "X"), 1);
        assert_eq!(playout("C", "Z"), 7);
    }
}

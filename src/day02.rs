use crate::{utils::read_lines, Solution};

#[derive(Default)]
pub struct Day2 {}

impl Solution for Day2 {
    type P1 = usize;
    type P2 = usize;

    fn part_1(&self) -> Self::P1 {
        let mut total_score = 0;
        for line in read_lines("src/data/day02").unwrap().flatten() {
            let (opponent, play) = line.split_at(1);
            total_score += score(opponent, play.trim_start());
        }
        total_score
    }

    fn part_2(&self) -> Self::P2 {
        let mut total_score = 0;
        for line in read_lines("src/data/day02").unwrap().flatten() {
            let (opponent, end_state) = line.split_at(1);
            total_score += play(opponent, end_state.trim_start());
        }
        total_score
    }
}

fn score(opponent: &str, play: &str) -> usize {
    match play {
        // draw
        "X" if opponent == "A" => 4,
        "Y" if opponent == "B" => 5,
        "Z" if opponent == "C" => 6,
        // win
        "X" if opponent == "C" => 7,
        "Y" if opponent == "A" => 8,
        "Z" if opponent == "B" => 9,
        // loss
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        // ???
        _ => 0,
    }
}

fn play(opponent: &str, end_state: &str) -> usize {
    match end_state {
        // lose
        "X" if opponent == "A" => 3,
        "X" if opponent == "B" => 1,
        "X" if opponent == "C" => 2,
        // draw
        "Y" if opponent == "A" => 4,
        "Y" if opponent == "B" => 5,
        "Y" if opponent == "C" => 6,
        // win
        "Z" if opponent == "A" => 8,
        "Z" if opponent == "B" => 9,
        "Z" if opponent == "C" => 7,
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
        assert_eq!(play("A", "Y"), 4);
        assert_eq!(play("B", "X"), 1);
        assert_eq!(play("C", "Z"), 7);
    }
}

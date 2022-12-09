use std::collections::HashSet;

use crate::Solution;

#[derive(Default)]
pub struct Day9 {}

impl Solution for Day9 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        tail_positions(include_str!("data/day9"), 2)
    }

    fn part_2(&self) -> Self::Result {
        tail_positions(include_str!("data/day9"), 10)
    }
}

type P = (i32, i32);

fn displace(a: P, b: &mut P) {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;

    if dx.abs() > 1 && dy.abs() > 1 {
        // diagonal
        *b = (b.0 + dx.signum(), b.1 + dy.signum());
    } else if dx.abs() > 1 {
        // horizontal
        *b = (b.0 + dx.signum(), a.1);
    } else if dy.abs() > 1 {
        // vertical
        *b = (a.0, b.1 + dy.signum());
    }
}

#[allow(clippy::cast_sign_loss)]
fn tail_positions(s: &str, knots: usize) -> usize {
    let motions = s.trim().lines().map(|line| {
        let (dir, steps_str) = line.split_once(' ').unwrap();
        (
            dir.chars().next().unwrap(),
            steps_str.parse::<i32>().unwrap(),
        )
    });

    let mut rope: Vec<P> = vec![(0, 0); knots];
    let mut visited: HashSet<P> = HashSet::new();
    visited.insert((0, 0));

    for (dir, steps) in motions {
        for _ in 0..steps {
            // move the head
            match dir {
                'R' => rope[0].0 += 1,
                'L' => rope[0].0 -= 1,
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                _ => panic!("invalid dir"),
            }
            // displace all the other knots, in order
            for i in 0..knots - 1 {
                displace(rope[i], &mut rope[i + 1]);

                if i == knots - 2 {
                    visited.insert(rope[i + 1]);
                }
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn tail_positions_short() {
        assert_eq!(tail_positions(TEST_INPUT_1, 2), 13);
    }

    #[test]
    fn tail_positions_long_1() {
        assert_eq!(tail_positions(TEST_INPUT_1, 10), 1);
    }

    const TEST_INPUT_2: &str = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn tail_positions_long_2() {
        assert_eq!(tail_positions(TEST_INPUT_2, 10), 36);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day9::new().part_1(), 6464);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(Day9::new().part_2(), 2604);
    }
}

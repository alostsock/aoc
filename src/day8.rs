use crate::Solution;
use std::cell::Cell;

#[derive(Default)]
pub struct Day8 {}

impl Solution for Day8 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let grid = parse_into_grid(include_str!("data/day8"));
        visible_from_outside(&grid)
    }

    fn part_2(&self) -> Self::Result {
        let grid = parse_into_grid(include_str!("data/day8"));
        highest_scenic_score(&grid)
    }
}

type Grid = Vec<Vec<u32>>;

fn parse_into_grid(s: &str) -> Grid {
    s.trim()
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[allow(unused_parens)]
fn visible_from_outside(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            let height = grid[row][col];

            // for the current tree, look up, down, left, right, respectively
            // to check if there exists a direction where every tree is shorter

            if ((0..row).all(|r| grid[r][col] < height)
                || (row + 1..rows).all(|r| grid[r][col] < height)
                || (0..col).all(|c| grid[row][c] < height)
                || (col + 1..cols).all(|c| grid[row][c] < height))
            {
                count += 1;
            }
        }
    }

    count
}

fn highest_scenic_score(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut highest_score = 0;

    for row in 0..rows {
        for col in 0..cols {
            let current_height = grid[row][col];

            // used in `take_while` to include the boundary element
            // https://github.com/rust-lang/rust/issues/62208
            let done = Cell::new(false);
            let check = |height: u32| -> bool {
                if done.get() {
                    return false;
                }
                if height >= current_height {
                    done.set(true);
                }
                true
            };

            // for the current tree, look in each direction and take trees with
            // shorter or equal height (inclusive)

            let up = (0..row).rev().take_while(|r| check(grid[*r][col])).count();

            done.set(false);
            let down = (row + 1..rows).take_while(|r| check(grid[*r][col])).count();

            done.set(false);
            let left = (0..col).rev().take_while(|c| check(grid[row][*c])).count();

            done.set(false);
            let right = (col + 1..cols).take_while(|c| check(grid[row][*c])).count();

            let score = up * down * left * right;

            highest_score = highest_score.max(score);
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
30373
25512
65332
33549
35390
";

    #[test]
    fn visible_from_outside_works() {
        let grid = parse_into_grid(TEST_INPUT);
        assert_eq!(visible_from_outside(&grid), 21);
    }

    #[test]
    fn highest_scenic_score_works() {
        let grid = parse_into_grid(TEST_INPUT);
        assert_eq!(highest_scenic_score(&grid), 8);
    }

    #[test]
    fn part_1() {
        assert_eq!(Day8::new().part_1(), 1690);
    }

    #[test]
    fn part_2() {
        assert_eq!(Day8::new().part_2(), 535_680);
    }
}

use std::collections::HashSet;

use crate::Solution;

#[derive(Default)]
pub struct Day14 {}

impl Solution for Day14 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day14");
        simulate(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day14");
        simulate_cycles(input)
    }
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn tilt(grid: &Grid) -> Grid {
    let total_rows = grid.len();
    let total_cols = grid[0].len();

    let mut tilted_grid: Grid = grid.clone();
    for j in 0..total_cols {
        for i in 0..total_rows {
            match tilted_grid[i][j] {
                'O' => {
                    if let Some(row) = find_unobstructed_row(&tilted_grid, i, j) {
                        tilted_grid[row][j] = 'O';
                        tilted_grid[i][j] = '.'
                    }
                }
                _ => (),
            }
        }
    }
    tilted_grid
}

fn find_unobstructed_row(grid: &Grid, i: usize, j: usize) -> Option<usize> {
    let mut row = i;
    for i in (0..i).rev() {
        if grid[i][j] != '.' {
            break;
        }
        row = i;
    }

    if row == i {
        None
    } else {
        Some(row)
    }
}

fn calculate_load(grid: &Grid) -> usize {
    let total_rows = grid.len();
    let total_cols = grid[0].len();

    let mut total_load = 0;

    for j in 0..total_cols {
        for i in 0..total_rows {
            if grid[i][j] == 'O' {
                total_load += total_rows - i;
            }
        }
    }

    total_load
}

fn simulate(input: &str) -> usize {
    let grid = parse(input);
    let tilted_grid = tilt(&grid);
    calculate_load(&tilted_grid)
}

fn rotate_clockwise(grid: &Grid) -> Grid {
    let mut rotated_grid = vec![vec!['.'; grid.len()]; grid[0].len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            rotated_grid[j][grid.len() - i - 1] = grid[i][j];
        }
    }

    rotated_grid
}

fn cycle(grid: Grid) -> Grid {
    [tilt, rotate_clockwise]
        .repeat(4)
        .into_iter()
        .fold(grid, |grid, f| f(&grid))
}

fn simulate_cycles(input: &str) -> usize {
    let mut grid = parse(input);
    let mut seen_scores = vec![];
    let mut seen_scores_set = HashSet::new();
    let mut cycle_start = 0;
    let mut cycle_scores = vec![];

    let mut nth_cycle = 0;
    loop {
        grid = cycle(grid);
        nth_cycle += 1;
        let score = calculate_load(&grid);

        seen_scores.push(score);
        if seen_scores_set.insert(score) {
            continue;
        }

        if cycle_scores.len() == 0 {
            cycle_start = nth_cycle;
            cycle_scores.push(score);
            continue;
        }

        if cycle_scores.len() > 1_000 && cycle_scores[0] == score {
            // cycle found
            break;
        } else if seen_scores[cycle_start - 1 + cycle_scores.len()] == score {
            // the cycle continues
            cycle_scores.push(score)
        } else {
            // the cycle is broken; start anew
            cycle_start = 0;
            cycle_scores.clear();
        };
    }

    // cycle_scores indices: _ _ 0 1 2 3 4 0 1 2 3 4
    // nth_cycle:            1 2 3 4 5 6 7 8 9

    let position_in_cycle = (1_000_000_000 - cycle_start) % cycle_scores.len();
    cycle_scores[position_in_cycle]
}

// fn print_grid(grid: &Grid) {
//     println!();
//     for row in grid {
//         let line: String = row.iter().collect();
//         println!("{line}");
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part_1() {
        assert_eq!(simulate(INPUT_1), 136);
    }

    #[test]
    fn part_2() {
        assert_eq!(simulate_cycles(INPUT_1), 64);
    }
}

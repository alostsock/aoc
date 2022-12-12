#![allow(const_item_mutation)]
use crate::Solution;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Day12 {}

impl Solution for Day12 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let (grid, start, end) = parse_grid(include_str!("data/day12"));
        find_path(&grid, vec![start], end).unwrap()
    }

    fn part_2(&self) -> Self::Result {
        let (grid, _, end) = parse_grid(include_str!("data/day12"));
        let starting_points = find_starting_points(&grid);
        find_path(&grid, starting_points, end).unwrap()
    }
}

type Grid = Vec<Vec<usize>>;

type P = (usize, usize);

fn parse_grid(input: &str) -> (Grid, P, P) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let grid = input
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| {
                    if char == 'S' {
                        start = (row, col);
                        'a' as usize
                    } else if char == 'E' {
                        end = (row, col);
                        'z' as usize
                    } else {
                        char as usize
                    }
                })
                .collect()
        })
        .collect();

    (grid, start, end)
}

fn find_path(grid: &Grid, starting_points: Vec<P>, end: P) -> Option<usize> {
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let mut to_visit: VecDeque<(P, usize)> = VecDeque::new();

    for start in starting_points {
        visited[start.0][start.1] = true;
        to_visit.push_back((start, 0));
    }

    while let Some((current, step)) = to_visit.pop_front() {
        if current == end {
            return Some(step);
        }

        let (row, col) = current;
        let height = grid[row][col];

        let (r, c) = (rows - 1, cols - 1);

        for next in [
            if row > 0 { Some((row - 1, col)) } else { None }, // up
            if row < r { Some((row + 1, col)) } else { None }, // down
            if col > 0 { Some((row, col - 1)) } else { None }, // left
            if col < c { Some((row, col + 1)) } else { None }, // right
        ]
        .iter()
        .flatten()
        {
            if visited[next.0][next.1] {
                continue;
            }

            let next_height = grid[next.0][next.1];

            if next_height < height || next_height - height <= 1 {
                visited[next.0][next.1] = true;
                to_visit.push_back((*next, step + 1));
            }
        }
    }

    None
}

fn find_starting_points(grid: &Grid) -> Vec<P> {
    let start_height = 'a' as usize;
    let mut starting_points: Vec<P> = vec![];

    for (r, row) in grid.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            if height == &start_height {
                starting_points.push((r, c));
            }
        }
    }

    starting_points
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn find_path_works() {
        let (grid, start, end) = parse_grid(TEST_INPUT);
        let least_steps = find_path(&grid, vec![start], end).unwrap();
        assert_eq!(least_steps, 31);
    }

    #[test]
    fn find_good_starting_point_works() {
        let (grid, _, end) = parse_grid(TEST_INPUT);
        let starting_points = find_starting_points(&grid);
        let least_steps = find_path(&grid, starting_points, end).unwrap();
        assert_eq!(least_steps, 29);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day12::new().part_1(), 449);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(Day12::new().part_2(), 443);
    }
}

#![allow(const_item_mutation)]
use crate::Solution;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Default)]
pub struct Day12 {}

impl Solution for Day12 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let (grid, start, end) = parse_grid(include_str!("data/day12"));
        find_path(&grid, start, end).unwrap()
    }

    fn part_2(&self) -> Self::Result {
        let (grid, _, end) = parse_grid(include_str!("data/day12"));
        find_good_starting_point(&grid, end)
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

fn find_path(grid: &Grid, start: P, end: P) -> Option<usize> {
    let mut visited: HashSet<P> = HashSet::new();
    visited.insert(start);
    let mut to_visit: VecDeque<(P, usize)> = VecDeque::new();
    to_visit.push_back((start, 0));

    let (r, c) = (grid.len() - 1, grid[0].len() - 1);

    while let Some((current, step)) = to_visit.pop_front() {
        if current == end {
            return Some(step);
        }

        let (row, col) = current;
        let height = grid[row][col];

        // up
        for next in [
            if row > 0 { Some((row - 1, col)) } else { None },
            if row < r { Some((row + 1, col)) } else { None },
            if col > 0 { Some((row, col - 1)) } else { None },
            if col < c { Some((row, col + 1)) } else { None },
        ]
        .iter()
        .flatten()
        {
            if visited.contains(next) {
                continue;
            }

            let next_height = grid[next.0][next.1];

            if next_height < height || next_height - height <= 1 {
                visited.insert(*next);
                to_visit.push_back((*next, step + 1));
            }
        }
    }

    None
}

fn find_good_starting_point(grid: &Grid, end: P) -> usize {
    let start_height = 'a' as usize;
    let mut starting_points: Vec<P> = vec![];

    for (r, row) in grid.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            if height == &start_height {
                starting_points.push((r, c));
            }
        }
    }

    let mut path_steps: Vec<usize> = starting_points
        .iter()
        .filter_map(|start| find_path(grid, *start, end))
        .collect();

    path_steps.sort_unstable();
    *path_steps.first().unwrap()
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
        let least_steps = find_path(&grid, start, end).unwrap();
        assert_eq!(least_steps, 31);
    }

    #[test]
    fn find_good_starting_point_works() {
        let (grid, _, end) = parse_grid(TEST_INPUT);
        let least_steps = find_good_starting_point(&grid, end);
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

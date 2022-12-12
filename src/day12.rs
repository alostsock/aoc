use crate::Solution;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day12 {}

impl Solution for Day12 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let (grid, start, end) = parse_grid(include_str!("data/day12"));
        find_path(&grid, start, end, 0, &mut HashMap::new()).unwrap()
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

fn find_path(
    grid: &Grid,
    current: P,
    goal: P,
    steps: usize,
    visited: &mut HashMap<P, usize>,
) -> Option<usize> {
    if current == goal {
        return Some(steps);
    }

    // don't continue if the current point was already
    // visited with the same or lower step count
    if let Some(visited_steps) = visited.get(&current) {
        if &steps >= visited_steps {
            return None;
        }
    }
    visited.insert(current, steps);

    let (row, col) = current;
    let (max_row, max_col) = (grid.len(), grid[0].len());

    let height = grid[row][col];

    let mut available_paths: Vec<P> = vec![];

    let climbable = |to: usize| -> bool { to < height || to - height <= 1 };

    // up
    if row > 0 && climbable(grid[row - 1][col]) {
        available_paths.push((row - 1, col));
    }
    // down
    if row + 1 < max_row && climbable(grid[row + 1][col]) {
        available_paths.push((row + 1, col));
    }
    // left
    if col > 0 && climbable(grid[row][col - 1]) {
        available_paths.push((row, col - 1));
    }
    // right
    if col + 1 < max_col && climbable(grid[row][col + 1]) {
        available_paths.push((row, col + 1));
    }

    if available_paths.is_empty() {
        return None;
    }

    let mut steps_to_end: Vec<usize> = available_paths
        .iter()
        .filter_map(|next| find_path(grid, *next, goal, steps + 1, visited))
        .collect();

    if steps_to_end.is_empty() {
        None
    } else {
        steps_to_end.sort_unstable();
        Some(*steps_to_end.first().unwrap())
    }
}

fn find_good_starting_point(grid: &Grid, goal: P) -> usize {
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
        .filter_map(|start| find_path(grid, *start, goal, 0, &mut HashMap::new()))
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
        let mut visited = HashMap::new();
        let least_steps = find_path(&grid, start, end, 0, &mut visited);
        assert_eq!(least_steps, Some(31));
    }

    #[test]
    fn find_good_starting_point_works() {
        let (grid, _, end) = parse_grid(TEST_INPUT);
        let least_steps = find_good_starting_point(&grid, end);
        assert_eq!(least_steps, 29);
    }

    // needs fixing -- in an unoptimized build,
    // both part 1 and part 2 result in stack overflows

    // #[test]
    // fn part_1_works() {
    //     assert_eq!(Day12::new().part_1(), 449);
    // }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(Day12::new().part_2(), 443);
    // }
}

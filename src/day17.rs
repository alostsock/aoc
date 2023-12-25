use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Default)]
pub struct Day17 {}

impl Solution for Day17 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day17");
        find_best_path(input)
    }

    fn part_2(&self) -> Self::Result {
        2023 * 25
    }
}

type Grid = Vec<Vec<char>>;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[rustfmt::skip]
fn next_position(grid: &Grid, (i, j): Position, direction: Direction) -> Option<Position> {
    let i_max = grid.len() - 1;
    let j_max = grid[0].len() - 1;

    match direction {
        Direction::Up => if i > 0 { Some((i - 1, j)) } else { None },
        Direction::Down => if i < i_max { Some((i + 1, j)) } else { None },
        Direction::Left => if j > 0 { Some((i, j - 1)) } else { None },
        Direction::Right => if j < j_max { Some((i, j + 1)) } else { None },
    }
}

fn end_position(grid: &Grid) -> Position {
    (grid.len() - 1, grid[0].len() - 1)
}

fn path(
    min_heat_loss: &mut usize,
    globally_visited_positions: &mut HashMap<(Position, Direction, usize), usize>,
    grid: &Grid,
    visited_positions: &HashSet<Position>,
    position: Position,
    direction: Direction,
    heat_loss: usize,
    consecutive_moves: usize,
) {
    use Direction::*;

    if consecutive_moves > 3 {
        return;
    }

    let mut visited_positions = visited_positions.clone();
    if !visited_positions.insert(position) {
        return;
    }

    let heat_loss = if visited_positions.len() == 1 {
        0
    } else {
        heat_loss + grid[position.0][position.1].to_digit(10).unwrap() as usize
    };

    if heat_loss > *min_heat_loss {
        return;
    }

    let global_key = (position, direction, consecutive_moves);

    if let Some(min_heat_loss_at_position) = globally_visited_positions.get(&global_key) {
        if heat_loss > *min_heat_loss_at_position {
            return;
        }

        globally_visited_positions.insert(global_key, heat_loss);
    } else {
        globally_visited_positions.insert(global_key, heat_loss);
    }

    if position == end_position(&grid) && heat_loss < *min_heat_loss {
        *min_heat_loss = heat_loss;
        return;
    }

    for next_direction in [Up, Right, Left, Down] {
        match (direction, next_direction) {
            (Up, Down) | (Right, Left) | (Left, Right) | (Down, Up) => continue,
            _ => (),
        };

        let Some(next_position) = next_position(grid, position, next_direction) else {
            continue;
        };

        let consecutive_moves = if direction == next_direction {
            consecutive_moves + 1
        } else {
            1
        };

        path(
            min_heat_loss,
            globally_visited_positions,
            grid,
            &visited_positions,
            next_position,
            next_direction,
            heat_loss,
            consecutive_moves,
        );
    }
}

fn find_best_path(input: &str) -> usize {
    let grid = parse(input);
    let visited_positions = HashSet::new();
    let mut min_heat_loss = usize::MAX;
    let mut globally_visited_positions = HashMap::new();

    path(
        &mut min_heat_loss,
        &mut globally_visited_positions,
        &grid,
        &visited_positions,
        (0, 0),
        Direction::Right,
        0,
        0,
    );

    min_heat_loss
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_1() {
        assert_eq!(find_best_path(INPUT_1), 102);
        // assert_eq!(Day17::new().part_1(), 2023);
    }
}

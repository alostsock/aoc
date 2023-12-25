use std::collections::{BinaryHeap, HashMap};

use crate::Solution;

#[derive(Default)]
pub struct Day17 {}

impl Solution for Day17 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day17");
        find_best_path(input, false)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day17");
        find_best_path(input, true)
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

type State = (Position, Direction, usize);

struct StatePriority {
    state: State,
    heat_loss: usize,
}

impl Ord for StatePriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for StatePriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.heat_loss.cmp(&self.heat_loss))
    }
}

impl PartialEq for StatePriority {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss == other.heat_loss
    }
}

impl Eq for StatePriority {}

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

fn find_best_path(input: &str, use_slow_steering: bool) -> usize {
    use Direction::*;

    let grid = parse(input);

    let mut visited_states: HashMap<State, usize> = HashMap::new();
    let mut visit_queue: BinaryHeap<StatePriority> = BinaryHeap::new();

    visit_queue.push(StatePriority {
        state: ((0, 0), Right, 0),
        heat_loss: 0,
    });

    loop {
        let Some(StatePriority { state, heat_loss }) = visit_queue.pop() else {
            break;
        };

        let (position, direction, consecutive_moves) = state;

        if !use_slow_steering {
            if consecutive_moves > 3 {
                continue;
            }
        } else {
            if consecutive_moves > 10 {
                continue;
            }
        }

        if position == end_position(&grid) {
            return heat_loss;
        }

        if let Some(&min_heat_loss_at_position) = visited_states.get(&state) {
            if heat_loss >= min_heat_loss_at_position {
                continue;
            }
        }

        visited_states.insert(state, heat_loss);

        for next_direction in [Up, Right, Left, Down] {
            // Skip backwards directions
            match (direction, next_direction) {
                (Up, Down) | (Right, Left) | (Left, Right) | (Down, Up) => continue,
                _ => (),
            };

            let Some(next_position) = next_position(&grid, position, next_direction) else {
                continue;
            };

            if use_slow_steering && direction != next_direction && consecutive_moves < 4 {
                continue;
            }

            let consecutive_moves = if direction == next_direction {
                consecutive_moves + 1
            } else {
                1
            };

            let heat_loss =
                heat_loss + grid[next_position.0][next_position.1].to_digit(10).unwrap() as usize;

            visit_queue.push(StatePriority {
                state: (next_position, next_direction, consecutive_moves),
                heat_loss,
            })
        }
    }

    unreachable!("path finding should always return a value");
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
        assert_eq!(find_best_path(INPUT_1, false), 102);
    }

    #[test]
    fn part_2() {
        assert_eq!(find_best_path(INPUT_1, true), 94);
    }
}

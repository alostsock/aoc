use std::collections::HashSet;

use crate::Solution;

#[derive(Default)]
pub struct Day16 {}

impl Solution for Day16 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day16");
        simulate_beam(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day16");
        find_best_beam(input)
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

fn beam(
    grid: &Grid,
    traveled_paths: &mut HashSet<(Position, Direction)>,
    position: Position,
    direction: Direction,
) {
    use Direction::*;

    let inserted = traveled_paths.insert((position, direction));
    if !inserted {
        return;
    }

    let current_tile = grid[position.0][position.1];
    let next_directions = match (current_tile, direction) {
        ('/', Up) => vec![Right],
        ('/', Right) => vec![Up],
        ('/', Down) => vec![Left],
        ('/', Left) => vec![Down],
        ('\\', Up) => vec![Left],
        ('\\', Right) => vec![Down],
        ('\\', Down) => vec![Right],
        ('\\', Left) => vec![Up],
        ('|', Left | Right) => vec![Up, Down],
        ('-', Up | Down) => vec![Left, Right],
        _ => vec![direction],
    };

    for direction in next_directions {
        let Some(position) = next_position(grid, position, direction) else {
            continue;
        };

        beam(grid, traveled_paths, position, direction);
    }
}

fn simulate_beam(input: &str) -> usize {
    let grid = parse(input);
    let mut traveled_paths = HashSet::new();
    beam(&grid, &mut traveled_paths, (0, 0), Direction::Right);

    let energized_tiles: HashSet<Position> = HashSet::from_iter(
        traveled_paths
            .into_iter()
            .map(|(position, _direction)| position),
    );
    energized_tiles.len()
}

fn starting_beams(grid: &Grid) -> Vec<(Position, Direction)> {
    use Direction::*;

    let i_max = grid.len() - 1;
    let j_max = grid[0].len() - 1;

    let mut beams = vec![];
    beams.extend((0..=i_max).map(|i| ((i, 0), Right)));
    beams.extend((0..=i_max).map(|i| ((i, j_max), Left)));
    beams.extend((0..=j_max).map(|j| ((0, j), Down)));
    beams.extend((0..=j_max).map(|j| ((i_max, j), Up)));
    beams
}

fn find_best_beam(input: &str) -> usize {
    let grid = parse(input);

    starting_beams(&grid)
        .into_iter()
        .map(|(position, direction)| {
            let mut traveled_paths = HashSet::new();
            beam(&grid, &mut traveled_paths, position, direction);

            let energized_tiles: HashSet<Position> = HashSet::from_iter(
                traveled_paths
                    .into_iter()
                    .map(|(position, _direction)| position),
            );
            energized_tiles.len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part_1() {
        assert_eq!(simulate_beam(INPUT_1), 46);
        assert_eq!(Day16::new().part_1(), 7608);
    }

    #[test]
    fn part_2() {
        assert_eq!(find_best_beam(INPUT_1), 51);
        // assert_eq!(Day16::new().part_2(), 8221);
    }
}

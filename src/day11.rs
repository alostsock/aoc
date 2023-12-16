use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Default)]
pub struct Day11 {}

impl Solution for Day11 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day11");
        Observation::from_str(input, 2).shortest_paths()
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day11");
        Observation::from_str(input, 1_000_000).shortest_paths()
    }
}

type Position = (usize, usize);

struct Observation {
    galaxy_positions: Vec<Position>,
}

impl Observation {
    fn from_str(input: &str, expansion_rate: usize) -> Self {
        let mut rows_with_galaxies = HashSet::new();
        let mut columns_with_galaxies = HashSet::new();

        for (i, row) in input.lines().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                if ch == '#' {
                    rows_with_galaxies.insert(i);
                    columns_with_galaxies.insert(j);
                }
            }
        }

        let mut galaxy_positions: Vec<Position> = vec![];

        let mut rows_added = 0;
        for (i, row) in input.lines().enumerate() {
            if !rows_with_galaxies.contains(&i) {
                rows_added += expansion_rate - 1;
                continue;
            }
            let mut columns_added = 0;
            for (j, ch) in row.chars().enumerate() {
                if !columns_with_galaxies.contains(&j) {
                    columns_added += expansion_rate - 1;
                    continue;
                }
                if ch == '#' {
                    galaxy_positions.push((i + rows_added, j + columns_added));
                }
            }
        }

        Self { galaxy_positions }
    }

    fn shortest_paths(&self) -> usize {
        let mut pairs: HashMap<(Position, Position), usize> = HashMap::new();

        for &a in &self.galaxy_positions {
            for &b in &self.galaxy_positions {
                if !pairs.contains_key(&(a, b)) && !pairs.contains_key(&(b, a)) {
                    pairs.insert((a, b), manhattan_distance(a, b));
                }
            }
        }

        pairs.values().sum()
    }
}

fn manhattan_distance((i0, j0): Position, (i1, j1): Position) -> usize {
    i1.abs_diff(i0) + j1.abs_diff(j0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(Observation::from_str(input, 2).shortest_paths(), 374);
        assert_eq!(Observation::from_str(input, 10).shortest_paths(), 1030);
        assert_eq!(Observation::from_str(input, 100).shortest_paths(), 8410);
    }
}

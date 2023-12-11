use std::collections::HashSet;

use crate::Solution;

#[derive(Default)]
pub struct Day10 {}

impl Solution for Day10 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day10");
        Tiles::from_str(input).find_furthest_distance_in_loop()
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day10");
        let tiles = Tiles::from_str(input);
        let loop_path = tiles
            .find_longest_loop_dfs(tiles.start_position, vec![])
            .unwrap();
        tiles.count_tiles_in_loop(loop_path)
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Pipe([bool; 4]),
    Ground,
    Start,
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            // top, right, bottom, left
            '|' => Self::Pipe([true, false, true, false]),
            '-' => Self::Pipe([false, true, false, true]),
            'L' => Self::Pipe([true, true, false, false]),
            'J' => Self::Pipe([true, false, false, true]),
            '7' => Self::Pipe([false, false, true, true]),
            'F' => Self::Pipe([false, true, true, false]),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("invalid tile: {ch}"),
        }
    }

    fn is_connected_to(&self, other: &Self, direction_index: usize) -> bool {
        match (self, other) {
            (Self::Pipe(self_openings), Self::Pipe(other_openings)) => {
                let self_has_opening = self_openings[direction_index];
                let other_has_opening = other_openings[(direction_index + 2) % 4];
                self_has_opening && other_has_opening
            }
            (Self::Start, Self::Pipe(_)) => true,
            (Self::Pipe(_), Self::Start) => true,
            _ => false,
        }
    }
}

type Position = (usize, usize);

struct Tiles {
    inner: Vec<Vec<Tile>>,
    i_max: usize,
    j_max: usize,
    start_position: Position,
}

impl Tiles {
    pub fn from_str(input: &str) -> Self {
        let mut start_position = (0, 0);
        let inner: Vec<Vec<_>> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, ch)| {
                        let tile = Tile::from_char(ch);

                        if tile == Tile::Start {
                            start_position = (i, j)
                        }

                        tile
                    })
                    .collect()
            })
            .collect();

        Self {
            i_max: inner.len() - 1,
            j_max: inner[0].len() - 1,
            inner,
            start_position,
        }
    }

    fn get(&self, (i, j): Position) -> &Tile {
        &self.inner[i][j]
    }

    fn connected_pipes(&self, position: Position) -> Vec<Position> {
        let current_tile = self.get(position);
        let (i, j) = position;
        let mut paths = vec![];

        if i > 0 && current_tile.is_connected_to(self.get((i - 1, j)), 0) {
            paths.push((i - 1, j))
        }
        if j < self.j_max && current_tile.is_connected_to(self.get((i, j + 1)), 1) {
            paths.push((i, j + 1))
        }
        if i < self.i_max && current_tile.is_connected_to(self.get((i + 1, j)), 2) {
            paths.push((i + 1, j))
        }
        if j > 0 && current_tile.is_connected_to(self.get((i, j - 1)), 3) {
            paths.push((i, j - 1))
        }

        paths
    }

    pub fn find_furthest_distance_in_loop(&self) -> usize {
        let longest_loop = self
            .find_longest_loop_dfs(self.start_position, vec![])
            .unwrap();
        longest_loop.len() / 2
    }

    fn find_longest_loop_dfs(
        &self,
        position: Position,
        mut path: Vec<Position>,
    ) -> Option<Vec<Position>> {
        if path.contains(&position) && position != self.start_position {
            return None;
        }

        path.push(position);

        if position == self.start_position && path.len() > 1 {
            return Some(path);
        }

        let pipe_positions = self.connected_pipes(position);

        if pipe_positions.len() == 0 {
            return None;
        }

        pipe_positions
            .into_iter()
            .flat_map(|position| self.find_longest_loop_dfs(position, path.clone()))
            .max_by(|a, b| a.len().cmp(&b.len()))
    }

    pub fn count_tiles_in_loop(&self, path: Vec<Position>) -> usize {
        let loop_tiles: HashSet<Position> = HashSet::from_iter(path);
        let start_tile = self.determine_start_tile();

        let mut count = 0;

        for i in 0..=self.i_max {
            for j in 0..=self.j_max {
                if self.is_tile_inside_loop(&(i, j), &loop_tiles, &start_tile) {
                    count += 1;
                }
            }
        }

        count
    }

    fn determine_start_tile(&self) -> Tile {
        let (i, j) = self.start_position;

        let mut connected_directions = [false; 4];

        if let Tile::Pipe([_, _, true, _]) = self.get((i - 1, j)) {
            connected_directions[0] = true
        }
        if let Tile::Pipe([_, _, _, true]) = self.get((i, j + 1)) {
            connected_directions[1] = true
        }
        if let Tile::Pipe([true, _, _, _]) = self.get((i + 1, j)) {
            connected_directions[2] = true
        }
        if let Tile::Pipe([_, true, _, _]) = self.get((i, j - 1)) {
            connected_directions[3] = true
        }

        Tile::Pipe(connected_directions)
    }

    fn is_tile_inside_loop(
        &self,
        position: &Position,
        loop_tiles: &HashSet<Position>,
        start_tile: &Tile,
    ) -> bool {
        if loop_tiles.contains(position) {
            return false;
        }

        let mut vertical_pipe_count = 0;
        let mut previous_pipe: Option<&Tile> = None;

        let i = position.0;
        for j in 0..position.1 {
            if !loop_tiles.contains(&(i, j)) {
                continue;
            }

            let tile = if (i, j) == self.start_position {
                start_tile
            } else {
                self.get((i, j))
            };

            use Tile::*;

            match (previous_pipe, tile) {
                (_, Pipe([true, _, true, _])) => vertical_pipe_count += 1,
                (_, Pipe([_, true, _, true])) => (),
                (Some(Pipe([true, true, _, _])), Pipe([_, _, true, true]))
                | (Some(Pipe([_, true, true, _])), Pipe([true, _, _, true])) => {
                    vertical_pipe_count += 1;
                    previous_pipe = None;
                }
                (Some(Pipe([true, true, _, _])), Pipe([true, _, _, true]))
                | (Some(Pipe([_, true, true, _])), Pipe([_, _, true, true])) => {
                    previous_pipe = None;
                }
                (_, pipe) => previous_pipe = Some(pipe),
            };
        }

        vertical_pipe_count % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(Tiles::from_str(input).find_furthest_distance_in_loop(), 8);
        // assert_eq!(Day10::new().part_1(), 7145);
    }

    #[test]
    fn part_2() {
        let input = "....................
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let tiles = Tiles::from_str(input);
        let loop_path = tiles
            .find_longest_loop_dfs(tiles.start_position, vec![])
            .unwrap();

        assert_eq!(tiles.count_tiles_in_loop(loop_path), 10);
        // assert_eq!(Day10::new().part_2(), 445);
    }
}

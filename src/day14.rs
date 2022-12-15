use crate::Solution;

#[derive(Default)]
pub struct Day14 {}

impl Solution for Day14 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let (mut cave, x_offset) = parse_cave_with_x_offset(include_str!("data/day14"), false);
        pour_sand(&mut cave, x_offset, false)
    }

    fn part_2(&self) -> Self::Result {
        let (mut cave, x_offset) = parse_cave_with_x_offset(include_str!("data/day14"), true);
        pour_sand(&mut cave, x_offset, true)
    }
}

type Grid = Vec<Vec<bool>>;
type P = (usize, usize);

const SOURCE: (usize, usize) = (500, 0);

fn parse_rocks(line: &str) -> Vec<P> {
    // line: 498,4 -> 498,6 -> 496,6
    let points: Vec<P> = line
        .split(" -> ")
        .map(|point_str| {
            let (x, y) = point_str.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut rocks: Vec<P> = vec![];

    // take each set of points pairwise,
    // and collect the points that lie between them
    for i in 0..points.len() - 1 {
        let (x0, y0) = points[i];
        let (x1, y1) = points[i + 1];
        if x0 < x1 {
            rocks.extend((x0..x1).map(|x| (x, y0)));
        } else if x1 < x0 {
            rocks.extend((x1 + 1..=x0).rev().map(|x| (x, y0)));
        } else if y0 < y1 {
            rocks.extend((y0..y1).map(|y| (x0, y)));
        } else if y1 < y0 {
            rocks.extend((y1 + 1..=y0).rev().map(|y| (x0, y)));
        } else {
            panic!("invalid line: {},{} -> {},{}", x0, y0, x1, y1);
        }
    }
    rocks.push(*points.last().unwrap());

    rocks
}

fn parse_cave_with_x_offset(input: &str, has_floor: bool) -> (Grid, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    let all_rocks: Vec<P> = input.trim().lines().flat_map(parse_rocks).collect();

    for (x, y) in &all_rocks {
        (min_x, max_x, max_y) = (min_x.min(*x), max_x.max(*x), max_y.max(*y));
    }

    let x_padding = if has_floor { max_y } else { 0 };
    let y_padding = if has_floor { 2 } else { 0 };

    let width = max_x - min_x + 1 + (x_padding * 2);
    let height = max_y + 1 + y_padding;

    let mut cave: Grid = vec![vec![false; width]; height];

    for (x, y) in all_rocks {
        cave[y][x - min_x + x_padding] = true;
    }

    if has_floor {
        *cave.last_mut().unwrap() = vec![true; width];
    }

    (cave, min_x - x_padding)
}

fn pour_sand(cave: &mut Grid, x_offset: usize, has_floor: bool) -> usize {
    let source_relative = (SOURCE.0 - x_offset, SOURCE.1);

    let y_limit = cave.len() - 1;
    let x_limit = cave[0].len() - 1;

    let mut units_of_sand = 0;

    loop {
        let (mut x, mut y): P = source_relative;

        loop {
            if (!has_floor && y == y_limit) || (has_floor && cave[y][x]) {
                return units_of_sand;
            }

            // bottom center
            if !cave[y + 1][x] {
                (x, y) = (x, y + 1);
                continue;
            }

            // bottom left
            if x == 0 {
                if has_floor {
                    cave[y][x] = true;
                    break;
                }
                return units_of_sand;
            }
            if !cave[y + 1][x - 1] {
                (x, y) = (x - 1, y + 1);
                continue;
            }

            // bottom right
            if x == x_limit {
                if has_floor {
                    cave[y][x] = true;
                    break;
                }
                return units_of_sand;
            }
            if !cave[y + 1][x + 1] {
                (x, y) = (x + 1, y + 1);
                continue;
            }

            // the sand has settled
            cave[y][x] = true;
            break;
        }

        units_of_sand += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn parse_rocks_works() {
        let rocks = parse_rocks("498,4 -> 498,6 -> 496,6");
        assert_eq!(
            rocks,
            vec![(498, 4), (498, 5), (498, 6), (497, 6), (496, 6)]
        );
    }

    #[test]
    fn pour_sand_works() {
        let (mut cave, x_offset) = parse_cave_with_x_offset(TEST_INPUT, false);
        let units_of_sand = pour_sand(&mut cave, x_offset, false);
        assert_eq!(units_of_sand, 24);
    }

    #[test]
    fn pour_sand_with_floor_works() {
        let (mut cave, x_offset) = parse_cave_with_x_offset(TEST_INPUT, true);
        let units_of_sand = pour_sand(&mut cave, x_offset, true);
        assert_eq!(units_of_sand, 93);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day14::new().part_1(), 1330);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(Day14::new().part_2(), 26139);
    }
}

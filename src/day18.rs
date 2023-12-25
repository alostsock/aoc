use crate::Solution;

#[derive(Default)]
pub struct Day18 {}

impl Solution for Day18 {
    type Result = isize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day18");
        area(input, false)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day18");
        area(input, true)
    }
}

type Position = (isize, isize);

fn process_line<'a>(line: &'a str, position: &'a Position, use_color: bool) -> Position {
    let (direction, rest) = line.split_once(' ').unwrap();
    let (distance, color) = rest.split_once(' ').unwrap();
    let (distance, direction) = if !use_color {
        let distance: isize = distance.parse().unwrap();
        (distance, direction)
    } else {
        let distance = isize::from_str_radix(&color[2..color.len() - 2], 16).unwrap();
        let direction = match color.chars().nth(color.len() - 2).unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!("invalid direction from color: {color}"),
        };
        (distance, direction)
    };
    let next_position = match direction {
        "R" => (position.0, position.1 + distance),
        "D" => (position.0 + distance, position.1),
        "L" => (position.0, position.1 - distance),
        "U" => (position.0 - distance, position.1),
        _ => panic!("invalid direction: {direction}"),
    };
    next_position
}

fn perimeter(input: &str, use_color: bool) -> isize {
    let mut perimeter = 0;
    let mut position = (0, 0);
    for line in input.lines() {
        let next_position = process_line(line, &position, use_color);
        let dx = position.0.abs_diff(next_position.0) as isize;
        let dy = position.1.abs_diff(next_position.1) as isize;
        perimeter += dx.max(dy);
        position = next_position;
    }
    perimeter
}

fn inner_area(input: &str, use_color: bool) -> isize {
    let mut position = (0, 0);
    let mut points: Vec<Position> = vec![(0, 0)];
    for line in input.lines() {
        let next_position = process_line(line, &position, use_color);
        position = next_position;
        points.push(position);
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula#Shoelace_formula
    let mut a = 0;
    for i in 1..points.len() {
        let (x0, y0) = points[i - 1];
        let (x1, y1) = points[i];
        a += x0 * y1 - y0 * x1
    }
    a.abs() / 2
}

fn area(input: &str, use_color: bool) -> isize {
    // We need to add ~half of the perimeter to the calculated area, since
    // the area formula assumes infinitely small points. For example, given a
    // 5x5 square, if we assume each point is located at the top left of each
    // tile, the area should be represented as the "A"s and perimeter as "P"s:
    //
    // A A A A P
    // A A A A P
    // A A A A P
    // A A A A P
    // P P P P P

    let inner_area = inner_area(input, use_color);
    let perimeter = perimeter(input, use_color);

    inner_area + (perimeter / 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn it_works() {
        assert_eq!(area(INPUT_1, false), 62);
        assert_eq!(area(INPUT_1, true), 952408144115);
    }
}

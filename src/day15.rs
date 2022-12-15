#![allow(clippy::cast_possible_wrap)]

use crate::Solution;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day15 {}

impl Solution for Day15 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let sensors_beacons = parse_sensors_beacons(include_str!("data/day15"));
        coverage_for_row(sensors_beacons, 2_000_000)
    }

    fn part_2(&self) -> Self::Result {
        todo!()
    }
}

type XY = (i32, i32);

fn parse_sensors_beacons(s: &str) -> Vec<(XY, XY, u32)> {
    s.trim()
        .lines()
        .map(|line| {
            line.split(['=', ',', ':'])
                .flat_map(str::parse)
                .collect::<Vec<i32>>()
        })
        .map(|nums| {
            let sensor = (nums[0], nums[1]);
            let beacon = (nums[2], nums[3]);
            (sensor, beacon, d_manhattan(sensor, beacon))
        })
        .collect()
}

fn d_manhattan(a: XY, b: XY) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn coverage_for_row(sensors_beacons: Vec<(XY, XY, u32)>, row: i32) -> usize {
    let (mut min_x, mut max_x) = (i32::MAX, 0);

    let mut beacons_in_row: HashSet<i32> = HashSet::new();

    for (s, b, d) in &sensors_beacons {
        let span_min_x = s.0 - (*d as i32);
        let span_max_x = s.0 + (*d as i32);
        min_x = min_x.min(span_min_x).min(b.0);
        max_x = max_x.max(span_max_x).max(b.0);

        if b.1 == row {
            beacons_in_row.insert(b.0);
        }
    }

    let mut seen: HashSet<i32> = HashSet::new();

    for (s, _, d) in sensors_beacons {
        let (x, y) = s;

        // check if the sensor is in range
        let x_span: i32 = if row.abs_diff(y) <= d {
            (d - row.abs_diff(y)) as i32
        } else {
            continue;
        };

        for x_row in (x - x_span)..=(x + x_span) {
            if x_row >= min_x && x_row <= max_x {
                seen.insert(x_row);
            }
        }
    }

    seen.difference(&beacons_in_row).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn parsing_works() {
        assert_eq!(
            parse_sensors_beacons("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            vec![((2, 18), (-2, 15), 7)]
        );
    }

    #[test]
    fn row_coverage_works() {
        assert_eq!(coverage_for_row(parse_sensors_beacons(TEST_INPUT), 10), 26);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day15::new().part_1(), 5_403_290);
    }
}

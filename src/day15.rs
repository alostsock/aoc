#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

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
        let sensors_beacons = parse_sensors_beacons(include_str!("data/day15"));
        let (x, y) = find_coverage_gap(&sensors_beacons, 4_000_000);
        (x as usize) * 4_000_000 + (y as usize)
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
    let mut beacons: HashSet<i32> = HashSet::new();
    let mut spans: Vec<(i32, i32)> = vec![];

    for (s, b, d) in sensors_beacons {
        // record the beacon position
        if b.1 == row {
            beacons.insert(b.0);
        }
        // check if the row is in range of the sensor
        if row.abs_diff(s.1) > d {
            continue;
        }
        let x_span: i32 = (d - row.abs_diff(s.1)) as i32;
        spans.push((s.0 - x_span, s.0 + x_span));
    }

    spans.sort_by(|a, b| a.0.cmp(&b.0));

    // merge spans
    let mut merged_spans: Vec<(i32, i32)> = vec![*spans.first().unwrap()];
    for (from, to) in spans.iter().skip(1) {
        let (_, prev_to) = merged_spans.last_mut().unwrap();

        if to < prev_to {
            continue;
        }

        if from <= prev_to {
            *prev_to = *to;
        } else {
            merged_spans.push((*from, *to));
        }
    }

    // find coverage
    let mut coverage: usize = 0;
    for (from, to) in merged_spans {
        let beacons_in_span = beacons.iter().filter(|b| **b >= from && **b <= to).count();
        coverage += (to - from) as usize + 1 - beacons_in_span;
    }
    coverage
}

fn coverage_gap_in_row(sensors_beacons: &Vec<(XY, XY, u32)>, row: i32, bound: i32) -> Option<i32> {
    let mut spans: Vec<(i32, i32)> = vec![];

    for (s, _, d) in sensors_beacons {
        // record the beacon position
        // check if the row is in range of the sensor
        if row.abs_diff(s.1) > *d {
            continue;
        }
        let x_span: i32 = (d - row.abs_diff(s.1)) as i32;
        let from = (s.0 - x_span).max(0);
        let to = (s.0 + x_span).min(bound);
        spans.push((from, to));
    }

    spans.sort_by(|a, b| a.0.cmp(&b.0));

    // merge spans
    let mut merged_spans: Vec<(i32, i32)> = vec![*spans.first().unwrap()];
    for (from, to) in spans.iter().skip(1) {
        let (_, prev_to) = merged_spans.last_mut().unwrap();

        if to < prev_to {
            continue;
        }

        if from <= prev_to {
            *prev_to = *to;
        } else {
            merged_spans.push((*from, *to));
        }
    }

    // check for a gap
    match &*merged_spans {
        [(_, b), (_, _)] => Some(b + 1),
        [(0, to)] if to == &bound => None,
        [(_, to)] if to == &bound => Some(0),
        [(0, _)] => Some(bound),
        _ => panic!("invalid span: {:?}", merged_spans),
    }
}

fn find_coverage_gap(sensors_beacons: &Vec<(XY, XY, u32)>, bound: i32) -> XY {
    for y in 0..bound {
        if let Some(x) = coverage_gap_in_row(sensors_beacons, y, bound) {
            return (x, y);
        }
    }
    panic!("couldn't find a gap")
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
    fn row_gap_works() {
        let sensors_beacons = parse_sensors_beacons(TEST_INPUT);
        let gap = find_coverage_gap(&sensors_beacons, 20);
        assert_eq!(gap, (14, 11));
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day15::new().part_1(), 5_403_290);
    }

    // this test is slow
    // #[test]
    // fn part_2_works() {
    //     assert_eq!(Day15::new().part_2(), 10_291_582_906_626);
    // }
}

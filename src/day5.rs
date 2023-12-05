use std::ops::Range;

use crate::Solution;

#[derive(Default)]
pub struct Day5 {}

impl Solution for Day5 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day5");
        find_min_location_v1(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day5");
        find_min_location_v2(input)
    }
}

#[derive(Debug)]
struct MapRange {
    sources: Range<usize>,
    destinations: Range<usize>,
}

impl MapRange {
    fn from_str(line: &str) -> Self {
        let params: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|param| param.parse().unwrap())
            .collect();

        let dest_start = params[0];
        let src_start = params[1];
        let length = params[2];

        Self {
            sources: src_start..(src_start + length),
            destinations: dest_start..(dest_start + length),
        }
    }

    fn lookup(&self, n: usize) -> Option<usize> {
        if !self.sources.contains(&n) {
            return None;
        }

        let offset = n - self.sources.start;
        Some(self.destinations.start + offset)
    }
}

#[derive(Debug)]
struct Map(Vec<MapRange>);

impl Map {
    fn from_str(map_str: &str) -> Self {
        Self(map_str.lines().map(MapRange::from_str).collect())
    }

    fn lookup(&self, source: usize) -> usize {
        for range in self.0.iter() {
            if let Some(destination) = range.lookup(source) {
                return destination;
            }
        }
        source
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>) {
    let (seeds_section, rest) = input.split_once("\n\n").unwrap();
    let (_label, seeds_str) = seeds_section.split_once(':').unwrap();
    let seeds = seeds_str
        .trim()
        .split_ascii_whitespace()
        .map(|seed_str| seed_str.parse().unwrap())
        .collect();

    let maps = rest
        .split("\n\n")
        .map(|section_with_header| {
            let (_header, section) = section_with_header.split_once('\n').unwrap();
            Map::from_str(section)
        })
        .collect();

    (seeds, maps)
}

fn find_min_location_v1(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);

    let locations = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |source, map| map.lookup(source)));

    locations.min().unwrap()
}

fn find_min_location_v2(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);

    let mut min_location = usize::MAX;

    for chunk in seeds.chunks(2) {
        let seed_start = chunk[0];
        let length = chunk[1];
        let seeds = seed_start..(seed_start + length);

        let min_chunk_location = seeds
            .into_iter()
            .map(|seed| maps.iter().fold(seed, |source, map| map.lookup(source)))
            .min()
            .unwrap();

        min_location = min_location.min(min_chunk_location);
    }

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1() {
        assert_eq!(find_min_location_v1(INPUT), 35);
        assert_eq!(Day5::new().part_1(), 600279879);
    }

    #[test]
    fn part_2() {
        assert_eq!(find_min_location_v2(INPUT), 46);
        // assert_eq!(Day5::new().part_2(), 20191102); // slow test
    }
}

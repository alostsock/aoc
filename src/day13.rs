use crate::Solution;

#[derive(Default)]
pub struct Day13 {}

impl Solution for Day13 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day13");
        summarize(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day13");
        summarize_smudged_variants(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Ash,
    Rock,
}

impl Item {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("invalid item: {ch}"),
        }
    }
}

struct Pattern {
    rows_bits: Vec<usize>,
    cols_bits: Vec<usize>,
}

impl Pattern {
    fn from_str(s: &str) -> Self {
        let inner: Vec<Vec<Item>> = s
            .lines()
            .map(|line| line.chars().map(Item::from_char).collect())
            .collect();

        let (rows_bits, cols_bits) = Self::create_bits(&inner);

        Self {
            rows_bits,
            cols_bits,
        }
    }

    fn smudged_variants_from_str(s: &str) -> impl Iterator<Item = Self> {
        let inner: Vec<Vec<Item>> = s
            .lines()
            .map(|line| line.chars().map(Item::from_char).collect())
            .collect();

        let item_count = inner.len() * inner[0].len();

        (0..item_count).map(move |n| {
            let mut flattened_inner: Vec<_> = inner.iter().cloned().flatten().collect();
            flattened_inner[n] = match flattened_inner[n] {
                Item::Ash => Item::Rock,
                Item::Rock => Item::Ash,
            };

            let smudged_inner: Vec<Vec<Item>> = flattened_inner
                .chunks(inner[0].len())
                .map(|chunk| chunk.to_vec())
                .collect();

            let (smudged_rows_bits, smudged_cols_bits) = Self::create_bits(&smudged_inner);

            Self {
                rows_bits: smudged_rows_bits,
                cols_bits: smudged_cols_bits,
            }
        })
    }

    fn create_bits(inner: &Vec<Vec<Item>>) -> (Vec<usize>, Vec<usize>) {
        let i_max = inner.len() - 1;
        let j_max = inner[0].len() - 1;

        let rows_bits = (0..=i_max)
            .map(|i| {
                (0..=j_max).fold(0, |bits, j| {
                    if inner[i][j] == Item::Rock {
                        bits | (1 << j)
                    } else {
                        bits
                    }
                })
            })
            .collect();

        let cols_bits = (0..=j_max)
            .map(|j| {
                (0..=i_max).fold(0, |bits, i| {
                    if inner[i][j] == Item::Rock {
                        bits | (1 << i)
                    } else {
                        bits
                    }
                })
            })
            .collect();

        (rows_bits, cols_bits)
    }

    fn find_reflection(
        &self,
        ignored_indices: (Option<usize>, Option<usize>),
    ) -> (Option<usize>, Option<usize>) {
        let mut vertical_reflection_index = 0;
        let mut vertical_reflection_size = 0;
        for index in 0..(self.cols_bits.len() - 1) {
            if ignored_indices.0 == Some(index) {
                continue;
            }
            for size in 1..=(index + 1) {
                let start = index - (size - 1);
                let end = index + size;
                if index + size > self.cols_bits.len() - 1
                    || self.cols_bits[start] != self.cols_bits[end]
                {
                    break;
                }
                if start == 0 || end == self.cols_bits.len() - 1 {
                    vertical_reflection_index = index;
                    vertical_reflection_size = size;
                }
            }
        }
        if vertical_reflection_size > 0 {
            return (Some(vertical_reflection_index), None);
        }

        let mut horizontal_reflection_index = 0;
        let mut horizontal_reflection_size = 0;
        for index in 0..(self.rows_bits.len() - 1) {
            if ignored_indices.1 == Some(index) {
                continue;
            }
            for size in 1..=(index + 1) {
                let start = index - (size - 1);
                let end = index + size;
                if index + size > self.rows_bits.len() - 1
                    || self.rows_bits[start] != self.rows_bits[end]
                {
                    break;
                }
                if start == 0 || end == self.rows_bits.len() - 1 {
                    horizontal_reflection_index = index;
                    horizontal_reflection_size = size;
                }
            }
        }
        if horizontal_reflection_size > 0 {
            return (None, Some(horizontal_reflection_index));
        }

        (None, None)
    }
}

fn summarize(input: &str) -> usize {
    input
        .split("\n\n")
        .map(
            |pattern_str| match Pattern::from_str(pattern_str).find_reflection((None, None)) {
                (Some(vertical_index), _) => vertical_index + 1,
                (_, Some(horizontal_index)) => (horizontal_index + 1) * 100,
                (None, None) => 0,
            },
        )
        .sum()
}

fn summarize_smudged_variants(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern_str| {
            let initial_result = Pattern::from_str(pattern_str).find_reflection((None, None));

            for smudged_pattern in Pattern::smudged_variants_from_str(pattern_str) {
                let smudged_result = smudged_pattern.find_reflection(initial_result);
                match smudged_result {
                    (Some(vertical_index), _) => return vertical_index + 1,
                    (_, Some(horizontal_index)) => return (horizontal_index + 1) * 100,
                    (None, None) => continue,
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    const INPUT_2: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.";

    #[test]
    fn part_1() {
        assert_eq!(summarize(INPUT_1), 405);
        assert_eq!(summarize(INPUT_2), 709);
        assert_eq!(Day13::new().part_1(), 29130);
    }

    #[test]
    fn part_2() {
        assert_eq!(summarize_smudged_variants(INPUT_1), 400);
        assert_eq!(summarize_smudged_variants(INPUT_2), 1400);
        assert_eq!(Day13::new().part_2(), 33438);
    }
}

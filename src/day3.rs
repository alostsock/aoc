use std::collections::HashSet;

use crate::Solution;

#[derive(Default)]
pub struct Day3 {}

impl Solution for Day3 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day3");
        part_numbers_sum(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day3");
        gear_ratios_sum(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Item {
    PartNumber(usize),
    Symbol(char),
    Empty,
}

struct Grid {
    inner: Vec<Vec<Item>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let inner: Vec<Vec<Item>> = input.lines().map(Self::items_from_line).collect();

        Self {
            width: inner[0].len(),
            height: inner.len(),
            inner,
        }
    }

    fn items_from_line(line: &str) -> Vec<Item> {
        let mut items = vec![Item::Empty; line.len()];

        let mut part_number_start_position = None;
        let mut part_number = 0;
        for (index, ch) in line.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                if part_number_start_position.is_none() {
                    part_number_start_position = Some(index);
                }
                part_number = part_number * 10 + digit as usize;

                continue;
            }

            if let Some(start_position) = part_number_start_position {
                for i in start_position..index {
                    items[i] = Item::PartNumber(part_number);
                }
                part_number_start_position = None;
                part_number = 0;
            }

            if ch == '.' {
                items[index] = Item::Empty
            } else {
                items[index] = Item::Symbol(ch);
            }
        }

        if let Some(start_position) = part_number_start_position {
            for i in start_position..line.len() {
                items[i] = Item::PartNumber(part_number);
            }
        }

        items
    }

    fn get(&self, x: usize, y: usize) -> Option<&Item> {
        self.inner.get(y).and_then(|row| row.get(x))
    }

    fn filter_adjacent_items<F>(&self, x: usize, y: usize, filter: F) -> impl Iterator<Item = &Item>
    where
        F: Fn(&&Item) -> bool,
    {
        let w = self.width - 1;
        let h = self.height - 1;

        #[rustfmt::skip]
        let surrounding_items = [
            if x > 0 && y > 0 { self.get(x - 1, y - 1) } else { None },
            if x > 0          { self.get(x - 1, y) } else { None },
            if x > 0 && y < h { self.get(x - 1, y + 1) } else { None },
            if          y > 0 { self.get(x, y - 1) } else { None },
            if          y < h { self.get(x, y + 1) } else { None },
            if x < w && y > 0 { self.get(x + 1, y - 1) } else { None },
            if x < w          { self.get(x + 1, y) } else { None },
            if x < w && y < h { self.get(x + 1, y + 1) } else { None },
        ];

        surrounding_items.into_iter().flatten().filter(filter)
    }
}

fn part_numbers_sum(input: &str) -> usize {
    let mut sum = 0;

    let grid = Grid::from_input(input);

    let mut part_number = 0;
    let mut has_symbol = false;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Item::PartNumber(number) = grid.get(x, y).unwrap() {
                part_number = *number;
                has_symbol = has_symbol
                    || grid
                        .filter_adjacent_items(x, y, |item| matches!(item, Item::Symbol(_)))
                        .next()
                        .is_some();
            } else {
                if has_symbol {
                    sum += part_number;
                }
                part_number = 0;
                has_symbol = false;
            }
        }
        if has_symbol {
            sum += part_number;
        }
        part_number = 0;
        has_symbol = false;
    }

    sum
}

fn gear_ratios_sum(input: &str) -> usize {
    let mut sum = 0;

    let grid = Grid::from_input(input);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y).unwrap() != &Item::Symbol('*') {
                continue;
            };

            let part_numbers: HashSet<&Item> = grid
                .filter_adjacent_items(x, y, |item| matches!(item, Item::PartNumber(_)))
                .collect();

            if part_numbers.len() != 2 {
                continue;
            }

            sum += part_numbers.iter().fold(1, |acc, item| {
                let Item::PartNumber(part_number) = item else {
                    panic!("expected part number");
                };
                acc * part_number
            });
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....*.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1() {
        assert_eq!(part_numbers_sum(INPUT), 4361);
    }

    #[test]
    fn part_2() {
        assert_eq!(gear_ratios_sum(INPUT), 467835);
    }
}

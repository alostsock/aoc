use crate::Solution;

#[derive(Default)]
pub struct Day5 {}

impl Solution for Day5 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day5");
        count_fresh_ingredients(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day5");
        count_all_fresh_ingredient_ids(input)
    }
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges_str, ingredients_str) = input
        .split_once("\n\n")
        .expect("valid ranges + ingredients");
    let ranges: Vec<(usize, usize)> = ranges_str
        .lines()
        .map(|line| {
            let (from_str, to_str) = line.split_once('-').expect("valid range");
            (
                from_str.parse().expect("valid range start"),
                to_str.parse().expect("valid range end"),
            )
        })
        .collect();

    let ingredients: Vec<usize> = ingredients_str
        .lines()
        .map(|s| s.parse().expect("valid ingredient"))
        .collect();

    (ranges, ingredients)
}

fn count_fresh_ingredients(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);

    ingredients
        .into_iter()
        .filter(|ingredient| {
            ranges
                .iter()
                .any(|(start, end)| ingredient >= start && ingredient <= end)
        })
        .count()
}

fn count_all_fresh_ingredient_ids(input: &str) -> usize {
    let (mut candidate_ranges, _) = parse_input(input);

    let mut distinct_ranges: Vec<(usize, usize)> = vec![];

    let mut mergeable = true;
    while mergeable {
        mergeable = false;

        for candidate_range in &candidate_ranges {
            let mergeable_position = distinct_ranges.iter().position(|distinct_range| {
                let unmergeable = candidate_range.1 < distinct_range.0 - 1
                    || candidate_range.0 > distinct_range.1 + 1;
                !unmergeable
            });
            if let Some(pos) = mergeable_position {
                mergeable = true;
                distinct_ranges[pos] = (
                    distinct_ranges[pos].0.min(candidate_range.0),
                    distinct_ranges[pos].1.max(candidate_range.1),
                )
            } else {
                distinct_ranges.push(*candidate_range);
            }
        }

        if mergeable {
            candidate_ranges = distinct_ranges;
            distinct_ranges = vec![];
        }
    }

    distinct_ranges
        .into_iter()
        .fold(0, |acc, (start, end)| acc + end - start + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(count_fresh_ingredients(input), 3);
    }

    #[test]
    fn part_2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(count_all_fresh_ingredient_ids(input), 14);
    }
}

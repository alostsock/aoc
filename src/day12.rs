use std::collections::HashMap;

use crate::Solution;

#[derive(Default)]
pub struct Day12 {}

impl Solution for Day12 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day12");
        count_arrangements(input, 1)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day12");
        count_arrangements(input, 5)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

type Arrangement = Vec<Condition>;

struct ConditionRecord {
    pattern: Arrangement,
    groups: Vec<usize>,
}

impl ConditionRecord {
    fn from_str(s: &str, copies: usize) -> Self {
        let (condition_str, groups_str) = s.split_once(' ').unwrap();

        let pattern: Arrangement = condition_str
            .chars()
            .map(|ch| match ch {
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                '?' => Condition::Unknown,
                _ => panic!("invalid condition: {ch}"),
            })
            .collect();
        let mut pattern_copied = pattern.clone();
        (1..copies).for_each(|_| {
            pattern_copied.push(Condition::Unknown);
            pattern_copied.extend(pattern.iter());
        });

        let groups: Vec<usize> = groups_str
            .split(',')
            .map(|ch| ch.parse().unwrap())
            .collect();
        let groups_copied = groups.repeat(copies);

        Self {
            pattern: pattern_copied,
            groups: groups_copied,
        }
    }

    fn count_arrangements(
        &self,
        cache: &mut HashMap<(usize, usize, usize), usize>,
        condition_index: usize,
        group_index: usize,
        damaged_count: usize,
    ) -> usize {
        if let Some(&count) = cache.get(&(condition_index, group_index, damaged_count)) {
            return count;
        }

        let group_count = *self.groups.get(group_index).unwrap_or(&0);

        let damaged_conditions_needed = (group_index..self.groups.len())
            .map(|i| *self.groups.get(i).unwrap_or(&0))
            .sum::<usize>()
            .saturating_sub(damaged_count);
        let conditions_remaining = self.pattern.len() - condition_index;
        if damaged_count > group_count || damaged_conditions_needed > conditions_remaining {
            return 0;
        }

        if condition_index >= self.pattern.len() {
            return 1;
        }

        let current_condition = self.pattern[condition_index];

        let handle_damaged = |cache| {
            if group_index == self.groups.len()
                || (damaged_count > 0 && damaged_count > group_count)
            {
                return 0;
            }

            self.count_arrangements(cache, condition_index + 1, group_index, damaged_count + 1)
        };

        let handle_operational = |cache| {
            if damaged_count > 0 && damaged_count != group_count {
                return 0;
            }

            let group_index = if damaged_count > 0 {
                group_index + 1
            } else {
                group_index
            };

            self.count_arrangements(cache, condition_index + 1, group_index, 0)
        };

        let count = match current_condition {
            Condition::Damaged => handle_damaged(cache),
            Condition::Operational => handle_operational(cache),
            Condition::Unknown => handle_damaged(cache) + handle_operational(cache),
        };

        cache
            .entry((condition_index, group_index, damaged_count))
            .or_insert(count);

        count
    }
}

fn count_arrangements(input: &str, copies: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let cache = &mut HashMap::default();
            ConditionRecord::from_str(line, copies).count_arrangements(cache, 0, 0, 0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1() {
        assert_eq!(count_arrangements(INPUT, 1), 21);
        assert_eq!(Day12::new().part_1(), 7118);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_arrangements(INPUT, 5), 525152);
        assert_eq!(Day12::new().part_2(), 7030194981795);
    }
}

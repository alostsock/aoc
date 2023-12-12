use crate::Solution;

#[derive(Default)]
pub struct Day12 {}

impl Solution for Day12 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day12");
        count_arrangements(input)
    }

    fn part_2(&self) -> Self::Result {
        2023 * 25
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
    min_damaged_count: usize,
}

impl ConditionRecord {
    fn from_str(s: &str) -> Self {
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

        let groups: Vec<usize> = groups_str
            .split(',')
            .map(|ch| ch.parse().unwrap())
            .collect();

        Self {
            pattern,
            min_damaged_count: groups.iter().sum(),
            groups,
        }
    }

    fn count_possible_arrangements(&self) -> usize {
        let mut arrangement_generator = ArrangementGenerator::from_pattern(&self.pattern);
        arrangement_generator
            .arrangements()
            .filter(|a| self.is_valid_arrangement(a))
            .count()
    }

    fn is_valid_arrangement(&self, arrangement: &Arrangement) -> bool {
        if arrangement
            .iter()
            .filter(|&&c| c == Condition::Damaged)
            .count()
            < self.min_damaged_count
        {
            return false;
        }

        let damaged_groups: Vec<_> = arrangement
            .split(|&condition| condition == Condition::Operational)
            .filter_map(|group| {
                if group.len() > 0 {
                    Some(group.len())
                } else {
                    None
                }
            })
            .collect();

        damaged_groups == self.groups
    }
}

struct ArrangementGenerator<'a> {
    pattern: &'a Arrangement,
    possibilities: usize,
}

impl<'a> ArrangementGenerator<'a> {
    fn from_pattern(arrangement: &'a Arrangement) -> Self {
        let unknowns_count = arrangement
            .iter()
            .filter(|&&a| a == Condition::Unknown)
            .count();

        Self {
            pattern: arrangement,
            possibilities: 2_usize.pow(unknowns_count.try_into().unwrap()),
        }
    }

    fn arrangements(&mut self) -> impl Iterator<Item = Arrangement> + '_ {
        (0..self.possibilities).map(|damaged_records_bit_mask| {
            let mut bit_mask_offset = 0;

            self.pattern
                .iter()
                .map(|&condition| match condition {
                    Condition::Unknown => {
                        let is_damaged = (damaged_records_bit_mask & (1 << bit_mask_offset)) > 0;

                        bit_mask_offset += 1;

                        if is_damaged {
                            Condition::Damaged
                        } else {
                            Condition::Operational
                        }
                    }
                    _ => condition,
                })
                .collect::<Arrangement>()
        })
    }
}

fn count_arrangements(input: &str) -> usize {
    input
        .lines()
        .map(|line| ConditionRecord::from_str(line).count_possible_arrangements())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(count_arrangements(input), 21);
        // assert_eq!(Day12::new().part_1(), 7118);
    }
}

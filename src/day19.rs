use std::collections::HashMap;

use crate::Solution;

#[derive(Default)]
pub struct Day19 {}

impl Solution for Day19 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day19");
        accepted_parts_rating_sum(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day19");
        count_ratings_combinations(input)
    }
}

type Part = Vec<usize>;

#[derive(Debug)]
enum Op<'a> {
    Lt(usize, usize, RetVal<'a>),
    Gt(usize, usize, RetVal<'a>),
    Return(RetVal<'a>),
}

#[derive(Debug)]
enum RetVal<'a> {
    Accept,
    Reject,
    Label(&'a str),
}

impl<'a> RetVal<'a> {
    fn from_str(s: &'a str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Label(s),
        }
    }
}

type Instructions<'a> = HashMap<&'a str, Vec<Op<'a>>>;

fn attr_index(attr_str: &str) -> usize {
    match attr_str {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("invalid attribute: {attr_str}"),
    }
}

fn parse(input: &str) -> (Instructions, Vec<Part>) {
    let (instructions_str, parts_str) = input.split_once("\n\n").unwrap();

    let mut instructions = HashMap::new();
    for instruction_str in instructions_str.lines() {
        let (label, ops_str) = instruction_str[0..instruction_str.len() - 1]
            .split_once('{')
            .unwrap();
        let ops: Vec<Op> = ops_str
            .split(',')
            .map(|op_str| {
                if op_str.contains('<') {
                    let (attr, rest) = op_str.split_once('<').unwrap();
                    let (value, return_value) = rest.split_once(':').unwrap();
                    Op::Lt(
                        attr_index(attr),
                        value.parse().unwrap(),
                        RetVal::from_str(return_value),
                    )
                } else if op_str.contains('>') {
                    let (attr, rest) = op_str.split_once('>').unwrap();
                    let (value, return_value) = rest.split_once(':').unwrap();
                    Op::Gt(
                        attr_index(attr),
                        value.parse().unwrap(),
                        RetVal::from_str(return_value),
                    )
                } else {
                    Op::Return(RetVal::from_str(op_str))
                }
            })
            .collect();

        instructions.insert(label, ops);
    }

    let mut parts = vec![];
    for line in parts_str.lines() {
        let part: Part = line[1..line.len() - 1]
            .split(['=', ','])
            .flat_map(|attr| attr.parse())
            .collect();
        parts.push(part);
    }

    (instructions, parts)
}

fn do_instruction(part: &Part, instructions: &Instructions, label: &str) -> bool {
    let process_retval = |retval: &RetVal| match retval {
        RetVal::Accept => true,
        RetVal::Reject => false,
        RetVal::Label(label) => do_instruction(part, instructions, label),
    };

    for op in &instructions[label] {
        match op {
            Op::Lt(attr_index, value, retval) => {
                if part[*attr_index] < *value {
                    return process_retval(retval);
                } else {
                    continue;
                }
            }
            Op::Gt(attr_index, value, retval) => {
                if part[*attr_index] > *value {
                    return process_retval(retval);
                } else {
                    continue;
                }
            }
            Op::Return(retval) => {
                return process_retval(retval);
            }
        }
    }

    unreachable!("instructions should always return a value");
}

fn combinations_from_range(rating_ranges: &Vec<(usize, usize)>) -> usize {
    rating_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .product()
}

fn ratings_combinations(
    rating_ranges: Vec<(usize, usize)>,
    instructions: &Instructions,
    label: &str,
) -> usize {
    let mut rating_ranges = rating_ranges.clone();
    let mut combinations = 0;

    let process_retval = |retval: &RetVal, rating_ranges: Vec<(usize, usize)>| match retval {
        RetVal::Accept => combinations_from_range(&rating_ranges),
        RetVal::Reject => 0,
        RetVal::Label(label) => ratings_combinations(rating_ranges, instructions, label),
    };

    for op in &instructions[label] {
        if rating_ranges.iter().any(|(start, end)| start >= end) {
            return 0;
        }

        match op {
            Op::Lt(attr_index, value, retval) => {
                let (start, end) = rating_ranges[*attr_index];
                // process valid range
                rating_ranges[*attr_index] = (start, end.min(value.saturating_sub(1)));
                combinations += process_retval(retval, rating_ranges.clone());
                // continue with invalid range
                rating_ranges[*attr_index] = (start.max(*value), end);
            }
            Op::Gt(attr_index, value, retval) => {
                let (start, end) = rating_ranges[*attr_index];
                // process valid range
                rating_ranges[*attr_index] = (start.max(value + 1), end);
                combinations += process_retval(retval, rating_ranges.clone());
                // continue with invalid range
                rating_ranges[*attr_index] = (start, end.min(*value))
            }
            Op::Return(retval) => {
                combinations += process_retval(retval, rating_ranges.clone());
            }
        }
    }

    combinations
}

fn accepted_parts_rating_sum(input: &str) -> usize {
    let (instructions, parts) = parse(input);
    parts
        .iter()
        .filter(|part| do_instruction(part, &instructions, "in"))
        .map(|part| part.iter().sum::<usize>())
        .sum()
}

fn count_ratings_combinations(input: &str) -> usize {
    let (instructions, _parts) = parse(input);
    let ranges = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    ratings_combinations(ranges, &instructions, "in")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_1() {
        assert_eq!(accepted_parts_rating_sum(INPUT_1), 19114)
    }

    #[test]
    fn part_2() {
        assert_eq!(count_ratings_combinations(INPUT_1), 167409079868000);
    }
}

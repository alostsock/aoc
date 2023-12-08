use core::panic;
use std::collections::HashMap;

use crate::Solution;

#[derive(Default)]
pub struct Day8 {}

impl Solution for Day8 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day8");
        let (instructions, nodes) = parse(input);
        count_steps_once(instructions, &nodes)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day8");
        let (instructions, nodes) = parse(input);
        count_steps_simultanenous(instructions, &nodes)
    }
}

type Nodes<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse(input: &str) -> (&str, Nodes) {
    let (instructions, nodes_str) = input.split_once("\n\n").unwrap();
    let mut nodes = HashMap::new();
    for node_str in nodes_str.lines() {
        let (node_name, rest) = node_str.split_once(" = ").unwrap();
        let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();
        nodes.insert(node_name, (left, right));
    }
    (instructions, nodes)
}

fn count_steps<F>(
    instructions: &str,
    nodes: &Nodes,
    start_node: &str,
    mut end_condition: F,
) -> usize
where
    F: FnMut(&str) -> bool,
{
    let mut steps = 0;
    let mut previous_node = start_node;
    for instruction in instructions.chars().cycle() {
        steps += 1;
        let current_node = match instruction {
            'L' => nodes[previous_node].0,
            'R' => nodes[previous_node].1,
            _ => panic!("invalid instruction: {instruction}"),
        };
        if end_condition(current_node) {
            break;
        }
        previous_node = current_node;
    }
    steps
}

fn count_steps_once(instructions: &str, nodes: &Nodes) -> usize {
    count_steps(instructions, nodes, "AAA", |node| node == "ZZZ")
}

fn count_steps_simultanenous(instructions: &str, nodes: &Nodes) -> usize {
    let start_nodes = nodes.keys().filter(|k| k.ends_with('A'));
    let steps_per_node =
        start_nodes.map(|node| count_steps(instructions, &nodes, node, |node| node.ends_with('Z')));
    steps_per_node.fold(1, lcm)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input_1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let input_2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (instructions, nodes) = parse(input_1);
        assert_eq!(count_steps_once(instructions, &nodes), 2);
        let (instructions, nodes) = parse(input_2);
        assert_eq!(count_steps_once(instructions, &nodes), 6);

        assert_eq!(Day8::new().part_1(), 17263);
    }

    #[test]
    fn part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (instructions, nodes) = parse(input);
        assert_eq!(count_steps_simultanenous(instructions, &nodes), 6);
        assert_eq!(Day8::new().part_2(), 14631604759649);
    }
}

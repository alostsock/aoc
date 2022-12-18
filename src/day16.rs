#![allow(clippy::cast_precision_loss)]

use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day16 {}

impl Solution for Day16 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let graph = Graph::from_str(include_str!("data/day16"));
        let valves_opened = vec![false; graph.nodes.len()];
        graph.find_max_pressure(graph.start_position, 30, 0, 0, &valves_opened)
    }

    fn part_2(&self) -> Self::Result {
        todo!()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    name: String,
    flow_rate: usize,
    children: Vec<usize>,
}

impl Node {
    fn new(name: &str, flow_rate: usize) -> Self {
        Self {
            name: String::from(name),
            flow_rate,
            children: vec![],
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    index_lookup: HashMap<String, usize>,
    distance_lookup: Vec<Vec<usize>>,
    start_position: usize,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: vec![],
            index_lookup: HashMap::new(),
            distance_lookup: vec![],
            start_position: 0,
        }
    }

    fn from_str(s: &str) -> Self {
        let re_names = Regex::new(r"[A-Z]{2}").unwrap();
        let re_flow_rate = Regex::new(r"\d+").unwrap();

        let valves: Vec<(&str, usize, Vec<&str>)> = s
            .trim()
            .lines()
            .map(|line| {
                let names: Vec<&str> = re_names.find_iter(line).map(|m| m.as_str()).collect();
                let flow_rate: usize = re_flow_rate.find(line).unwrap().as_str().parse().unwrap();
                (names[0], flow_rate, names[1..].to_vec())
            })
            .collect();

        let mut graph = Self::new();

        // register individual valves
        for (name, flow_rate, _) in &valves {
            let index = graph.nodes.len();
            if *name == "AA" {
                graph.start_position = index;
            }
            graph.index_lookup.insert(String::from(*name), index);
            graph.nodes.push(Node::new(name, *flow_rate));
        }

        // add tunnel associations
        for (name, _, tunnels) in valves {
            let valve_index = graph.index_lookup[name];
            let tunnel_indices: Vec<usize> =
                tunnels.iter().map(|t| graph.index_lookup[*t]).collect();
            graph.node_mut(valve_index).children.extend(tunnel_indices);
        }

        // discover distances between nodes ahead of time
        for index in 0..graph.nodes.len() {
            graph.distance_lookup.push(graph.find_distances(index));
        }

        graph
    }

    fn node(&self, i: usize) -> &Node {
        self.nodes.get(i).unwrap()
    }

    fn node_mut(&mut self, i: usize) -> &mut Node {
        self.nodes.get_mut(i).unwrap()
    }

    // for a given start_index, use bfs to find the shortest distance each other node
    fn find_distances(&self, start_index: usize) -> Vec<usize> {
        let node_count = self.nodes.len();
        let mut distances: Vec<usize> = vec![0; node_count];

        let mut seen: Vec<bool> = vec![false; node_count];
        seen[start_index] = true;
        let mut seen_count = 1;

        let mut to_visit: Vec<usize> = self.node(start_index).children.clone();

        let mut distance_traveled = 0;

        while seen_count < node_count {
            distance_traveled += 1;

            to_visit = to_visit
                .iter()
                .filter_map(|index| {
                    if !seen[*index] {
                        seen[*index] = true;
                        seen_count += 1;
                        distances[*index] = distance_traveled;
                    }

                    if *index == start_index {
                        None
                    } else {
                        Some(self.node(*index).children.clone())
                    }
                })
                .flatten()
                .collect();
        }

        distances
    }

    // use dfs to go through all possible valve-opening combinations
    fn find_max_pressure(
        &self,
        position: usize,
        time_remaining: usize,
        global_flow_rate: usize,
        pressure_released: usize,
        valves_opened: &[bool],
    ) -> usize {
        let pressure_released = pressure_released + global_flow_rate;

        let candidate_max = self.distance_lookup[position]
            .iter()
            .enumerate()
            .filter_map(|(valve_index, distance)| {
                let flow_rate = self.node(valve_index).flow_rate;

                if valves_opened[valve_index] || flow_rate == 0 || distance + 1 >= time_remaining {
                    None
                } else {
                    let mut valves_opened = valves_opened.to_owned();
                    valves_opened[valve_index] = true;

                    Some(self.find_max_pressure(
                        valve_index,
                        time_remaining - (distance + 1),
                        global_flow_rate + flow_rate,
                        pressure_released + global_flow_rate * distance,
                        &valves_opened,
                    ))
                }
            })
            .max();

        if let Some(max_pressure) = candidate_max {
            max_pressure
        } else {
            pressure_released + global_flow_rate * (time_remaining - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    // #[test]
    // fn input_parsing_works() {
    //     dbg!(Graph::from_str(TEST_INPUT));
    // }

    #[test]
    fn release_pressure_works() {
        let graph = Graph::from_str(TEST_INPUT);
        let valves_opened = vec![false; graph.nodes.len()];
        let pressure = graph.find_max_pressure(graph.start_position, 30, 0, 0, &valves_opened);
        assert_eq!(pressure, 1651);
    }

    // #[test]
    // fn part_1_works() {
    //     assert_eq!(Day16::new().part_1(), 1896);
    // }
}

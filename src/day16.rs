#![allow(clippy::cast_precision_loss)]

use crate::Solution;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Day16 {}

impl Solution for Day16 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let mut graph = Graph::from_str(include_str!("data/day16"));
        graph.find_max_pressure(graph.start_position, 1, 0, 0, &[])
    }

    fn part_2(&self) -> Self::Result {
        2022 * 25
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
        let mut seen: HashSet<usize> = HashSet::new();
        let mut distances: Vec<usize> = vec![0; node_count];
        let mut to_visit: Vec<usize> = vec![];

        seen.insert(start_index);
        to_visit.extend(self.node(start_index).children.iter());

        let mut distance_traveled = 0;

        while seen.len() < node_count {
            distance_traveled += 1;

            to_visit = to_visit
                .iter()
                .filter_map(|index| {
                    if !seen.contains(index) {
                        seen.insert(*index);
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
        &mut self,
        position: usize,
        time: usize,
        global_flow_rate: usize,
        pressure_released: usize,
        valves_opened: &[usize],
    ) -> usize {
        let pressure_released = pressure_released + global_flow_rate;

        assert!(time <= 30);

        if time == 30 {
            return pressure_released;
        }

        let distances = self.distance_lookup[position].clone();

        let candidate_max = distances
            .iter()
            .enumerate()
            .filter_map(|(valve_index, distance)| {
                let flow_rate = self.node(valve_index).flow_rate;

                if valves_opened.contains(&valve_index) || flow_rate == 0 || time + distance >= 30 {
                    None
                } else {
                    let mut valves_opened = valves_opened.to_owned();
                    valves_opened.push(valve_index);

                    Some(self.find_max_pressure(
                        valve_index,
                        time + distance + 1,
                        global_flow_rate + flow_rate,
                        pressure_released + global_flow_rate * distance,
                        &valves_opened,
                    ))
                }
            })
            .max();

        if let Some(best_candidate) = candidate_max {
            best_candidate
        } else {
            pressure_released + global_flow_rate * (30 - time)
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
        let mut graph = Graph::from_str(TEST_INPUT);
        let pressure = graph.find_max_pressure(graph.start_position, 1, 0, 0, &[]);
        assert_eq!(pressure, 1651);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day16::new().part_1(), 1896);
    }
}
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day16 {}

impl Solution for Day16 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let graph = Graph::from_str(include_str!("data/day16"));
        let end_state = graph.find_max_pressure(State {
            position: graph.start_position,
            time_remaining: 30,
            global_flow_rate: 0,
            pressure_released: 0,
            disabled_valves: vec![false; graph.nodes.len()],
        });
        end_state.pressure_released
    }

    fn part_2(&self) -> Self::Result {
        let graph = Graph::from_str(include_str!("data/day16"));
        graph.find_max_pressure_with_elephant()
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
struct State {
    position: usize,
    time_remaining: usize,
    global_flow_rate: usize,
    pressure_released: usize,
    disabled_valves: Vec<bool>,
}

impl State {
    fn open_valve(&self, index: usize, flow_rate: usize, distance: usize) -> Self {
        let mut disabled_valves = self.disabled_valves.clone();
        disabled_valves[index] = true;

        Self {
            position: index,
            time_remaining: self.time_remaining - (distance + 1),
            global_flow_rate: self.global_flow_rate + flow_rate,
            pressure_released: self.pressure_released + self.global_flow_rate * distance,
            disabled_valves,
        }
    }

    fn wait_out_timer(&self) -> Self {
        let final_pressure =
            self.pressure_released + self.global_flow_rate * (self.time_remaining - 1);

        Self {
            position: self.position,
            time_remaining: 1,
            global_flow_rate: self.global_flow_rate,
            pressure_released: final_pressure,
            disabled_valves: self.disabled_valves.clone(),
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
    fn find_max_pressure(&self, mut state: State) -> State {
        state.pressure_released += state.global_flow_rate;

        let candidate_max = self.distance_lookup[state.position]
            .iter()
            .enumerate()
            .filter_map(|(valve_index, distance)| {
                let flow_rate = self.node(valve_index).flow_rate;

                if state.disabled_valves[valve_index]
                    || flow_rate == 0
                    || distance + 1 >= state.time_remaining
                {
                    return None;
                }

                let next_state = state.open_valve(valve_index, flow_rate, *distance);
                Some(self.find_max_pressure(next_state))
            })
            .max_by_key(|state| state.pressure_released);

        if let Some(end_state) = candidate_max {
            end_state
        } else {
            state.wait_out_timer()
        }
    }

    fn disabled_valves_from_bitmask(&self, mask: u16, openable_valves: &[usize]) -> Vec<bool> {
        let mut disabled_valves = vec![false; self.nodes.len()];

        for (i, valve_index) in openable_valves.iter().enumerate() {
            let is_disabled = (mask >> i & 1) == 1;
            disabled_valves[*valve_index] = is_disabled;
        }

        disabled_valves
    }

    fn inverse_bitmask(mask: u16, length: usize) -> u16 {
        let unused_bits = 16 - length;
        (!mask << unused_bits) >> unused_bits
    }

    // we need to run part 1 for both the person and elephant, with different
    // combinations of valves disabled at the start.
    fn find_max_pressure_with_elephant(&self) -> usize {
        let openable_valves: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| if node.flow_rate > 0 { Some(i) } else { None })
            .collect();

        let mut max_pressure = 0;

        // run through all possible disabled valve states. in the input, there's
        // only 15 openable valves, so a u16 should be enough

        let valve_count = openable_valves.len();
        let total_states = 2_u16.pow(valve_count as u32);

        let mut cached_bitmask_results: HashMap<u16, usize> = HashMap::new();

        for bitmask in 0..total_states {
            let inverse_bitmask = Self::inverse_bitmask(bitmask, valve_count);

            let combined_pressure = [bitmask, inverse_bitmask]
                .map(|mask| {
                    if let Some(cached_pressure) = cached_bitmask_results.get(&mask) {
                        *cached_pressure
                    } else {
                        let disabled_valves =
                            self.disabled_valves_from_bitmask(mask, &openable_valves);

                        let pressure = self
                            .find_max_pressure(State {
                                position: self.start_position,
                                time_remaining: 26,
                                global_flow_rate: 0,
                                pressure_released: 0,
                                disabled_valves,
                            })
                            .pressure_released;

                        cached_bitmask_results.insert(mask, pressure);
                        pressure
                    }
                })
                .iter()
                .sum();

            max_pressure = max_pressure.max(combined_pressure);
        }

        max_pressure
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
    fn find_max_pressure_works() {
        let graph = Graph::from_str(TEST_INPUT);
        let end_state = graph.find_max_pressure(State {
            position: graph.start_position,
            time_remaining: 30,
            global_flow_rate: 0,
            pressure_released: 0,
            disabled_valves: vec![false; graph.nodes.len()],
        });
        assert_eq!(end_state.pressure_released, 1651);
    }

    #[test]
    fn find_max_pressure_with_elephant_works() {
        let graph = Graph::from_str(TEST_INPUT);
        let max_pressure = graph.find_max_pressure_with_elephant();
        assert_eq!(max_pressure, 1707);
    }

    // #[test]
    // fn part_1_works() {
    //     assert_eq!(Day16::new().part_1(), 1896);
    // }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(Day16::new().part_2(), 2576);
    // }
}

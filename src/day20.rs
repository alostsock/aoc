use std::collections::{HashMap, HashSet, VecDeque};

use crate::Solution;

#[derive(Default)]
pub struct Day20 {}

impl Solution for Day20 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day20");
        simulate_button_presses(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day20");
        count_presses_for_rx(input)
    }
}

trait Signalable<'a> {
    fn label(&self) -> &str;
    fn destinations(&self) -> &Vec<&'a str>;
    fn send(&mut self, signal: bool, source: &'a str) -> Option<bool>;
}

#[derive(Debug)]
struct Broadcaster<'a> {
    label: &'a str,
    destinations: Vec<&'a str>,
}

impl<'a> Signalable<'a> for Broadcaster<'a> {
    fn label(&self) -> &str {
        self.label
    }
    fn destinations(&self) -> &Vec<&'a str> {
        &self.destinations
    }
    fn send(&mut self, _signal: bool, _source: &'a str) -> Option<bool> {
        Some(false)
    }
}

#[derive(Debug)]
struct FlipFlop<'a> {
    label: &'a str,
    state: bool,
    destinations: Vec<&'a str>,
}

impl<'a> Signalable<'a> for FlipFlop<'a> {
    fn label(&self) -> &str {
        self.label
    }
    fn destinations(&self) -> &Vec<&'a str> {
        &self.destinations
    }
    fn send(&mut self, signal: bool, _source: &str) -> Option<bool> {
        if !signal {
            self.state = !self.state;
            Some(self.state)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction<'a> {
    label: &'a str,
    state: HashSet<&'a str>,
    sources: Vec<&'a str>,
    destinations: Vec<&'a str>,
}

impl<'a> Signalable<'a> for Conjunction<'a> {
    fn label(&self) -> &'a str {
        self.label
    }
    fn destinations(&self) -> &Vec<&'a str> {
        &self.destinations
    }
    fn send(&mut self, signal: bool, source: &'a str) -> Option<bool> {
        if signal {
            self.state.insert(source);
        } else {
            self.state.remove(source);
        }
        Some(self.state.len() != self.sources.len())
    }
}

#[derive(Debug)]
enum Module<'a> {
    Broadcaster(Broadcaster<'a>),
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
}

// So much boilerplate...

impl<'a> Signalable<'a> for Module<'a> {
    fn label(&self) -> &str {
        match self {
            Self::Broadcaster(m) => m.label(),
            Self::FlipFlop(m) => m.label(),
            Self::Conjunction(m) => m.label(),
        }
    }
    fn destinations(&self) -> &Vec<&'a str> {
        match self {
            Self::Broadcaster(m) => m.destinations(),
            Self::FlipFlop(m) => m.destinations(),
            Self::Conjunction(m) => m.destinations(),
        }
    }
    fn send(&mut self, signal: bool, source: &'a str) -> Option<bool> {
        match self {
            Self::Broadcaster(m) => m.send(signal, source),
            Self::FlipFlop(m) => m.send(signal, source),
            Self::Conjunction(m) => m.send(signal, source),
        }
    }
}

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    let mut modules = HashMap::new();

    // Keep track of these so we can assign sources to them later
    let mut conjunction_modules_sources: HashMap<&str, Vec<&str>> = HashMap::new();

    for module_str in input.lines() {
        let (mut label, destinations_str) = module_str.split_once(" -> ").unwrap();
        let destinations = destinations_str.split(", ").collect();
        let module = match label.chars().next().unwrap() {
            'b' => Module::Broadcaster(Broadcaster {
                label,
                destinations,
            }),
            '%' => {
                label = &label[1..label.len()];
                Module::FlipFlop(FlipFlop {
                    label,
                    state: false,
                    destinations,
                })
            }
            '&' => {
                label = &label[1..label.len()];
                conjunction_modules_sources.insert(label, vec![]);
                Module::Conjunction(Conjunction {
                    label,
                    state: HashSet::new(),
                    sources: vec![],
                    destinations,
                })
            }
            _ => panic!("invalid module: {label}"),
        };
        modules.insert(label, module);
    }

    for (source_label, module) in modules.iter() {
        for destination_label in module.destinations() {
            conjunction_modules_sources
                .entry(&destination_label)
                .and_modify(|sources| sources.push(source_label));
        }
    }

    for (label, sources) in conjunction_modules_sources {
        let conjunction_module = modules.get_mut(label).unwrap();
        let Module::Conjunction(module) = conjunction_module else {
            panic!("invalid conjunction module: {label}");
        };
        module.sources = sources;
    }

    modules
}

fn broadcast<F>(modules: &mut HashMap<&str, Module>, check_output: &mut F) -> (usize, usize)
where
    F: FnMut((bool, &str)),
{
    let mut signals_sent = (0, 0);

    let mut signals_queue: VecDeque<(bool, &str, &str)> = VecDeque::new();
    signals_queue.push_back((false, "button", "broadcaster"));

    while signals_queue.len() > 0 {
        let (signal, source, target) = signals_queue.pop_front().unwrap();

        check_output((signal, source));

        // println!(
        //     "{} {} {}",
        //     source,
        //     if signal { "high" } else { "low" },
        //     target
        // );

        if signal {
            signals_sent.1 += 1;
        } else {
            signals_sent.0 += 1;
        }

        let Some(module) = modules.get_mut(target) else {
            continue;
        };
        let Some(output_signal) = module.send(signal, source) else {
            continue;
        };
        for destination in module.destinations() {
            signals_queue.push_back((output_signal, target, destination));
        }
    }

    signals_sent
}

fn simulate_button_presses(input: &str) -> usize {
    let modules = &mut parse_modules(input);
    let mut total_low_signals_sent = 0;
    let mut total_high_signals_sent = 0;
    for _ in 0..1000 {
        let (low_signals_sent, high_signals_sent) = broadcast(modules, &mut |_| {});
        total_low_signals_sent += low_signals_sent;
        total_high_signals_sent += high_signals_sent;
    }
    total_low_signals_sent * total_high_signals_sent
}

fn count_presses_for_rx(input: &str) -> usize {
    let modules = &mut parse_modules(input);

    // "rx" depends on a conjunction module "lg" that is sourced by
    // "vg", "nb", "vc", and "ls". So we need to keep track of how
    // many presses it takes to turn on each of these, then take the
    // product of those presses.

    let mut presses = [0, 0, 0, 0];
    let mut current_presses = 0;
    while presses.iter().any(|&n| n == 0) {
        current_presses += 1;

        broadcast(modules, &mut |(signal, source)| {
            if signal && presses[0] == 0 && source == "vg" {
                presses[0] = current_presses;
            }
            if signal && presses[1] == 0 && source == "nb" {
                presses[1] = current_presses;
            }
            if signal && presses[2] == 0 && source == "vc" {
                presses[2] = current_presses;
            }
            if signal && presses[3] == 0 && source == "ls" {
                presses[3] = current_presses;
            }
        });
    }
    presses.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part_1() {
        assert_eq!(simulate_button_presses(INPUT_1), 32000000);
        assert_eq!(simulate_button_presses(INPUT_2), 11687500);
    }
}

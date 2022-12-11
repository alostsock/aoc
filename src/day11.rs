use crate::Solution;

#[derive(Default)]
pub struct Day11 {}

impl Solution for Day11 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let mut monkeys = parse_monkeys(include_str!("data/day11"));
        do_monkey_business(&mut monkeys, 20, false);

        let mut items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
        items_inspected.sort_unstable();
        items_inspected.reverse();
        items_inspected.iter().take(2).product()
    }

    fn part_2(&self) -> Self::Result {
        let mut monkeys = parse_monkeys(include_str!("data/day11"));
        do_monkey_business(&mut monkeys, 10_000, true);

        let mut items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
        items_inspected.sort_unstable();
        items_inspected.reverse();
        items_inspected.iter().take(2).product()
    }
}

type F = Box<dyn Fn(usize) -> usize>;

struct Monkey {
    items: Vec<usize>,
    items_inspected: usize,
    operation: F,
    test: F,
    divisor: usize,
}

impl Monkey {
    fn from_str(s: &str) -> Self {
        let lines: Vec<&str> = s
            .lines()
            .filter_map(|line| line.split(':').last())
            .collect();

        let items: Vec<usize> = lines[1].trim().split(", ").flat_map(str::parse).collect();

        let operation_parts: Vec<&str> = lines[2]
            .split(" = ")
            .last()
            .unwrap()
            .split_whitespace()
            .collect();

        let [divisor, true_target, false_target]: [usize; 3] = lines[3..]
            .iter()
            .map(|line| line.split(' ').find_map(|s| s.parse().ok()).unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        let operation: F = match operation_parts.as_slice() {
            ["old", "*", "old"] => Box::new(|old: usize| old * old),
            ["old", "*", x] => {
                let x = x.parse::<usize>().unwrap();
                Box::new(move |old| old * x)
            }
            ["old", "+", "old"] => Box::new(|old| old + old),
            ["old", "+", x] => {
                let x = x.parse::<usize>().unwrap();
                Box::new(move |old| old + x)
            }
            _ => panic!("invalid operation"),
        };

        let test = Box::new(move |worry_level: usize| {
            if worry_level % divisor == 0 {
                true_target
            } else {
                false_target
            }
        });

        Monkey {
            items,
            items_inspected: 0,
            operation,
            test,
            divisor,
        }
    }
}

fn parse_monkeys(s: &str) -> Vec<Monkey> {
    s.trim().split("\n\n").map(Monkey::from_str).collect()
}

fn do_monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, is_very_worried: bool) {
    let common_divisor: usize = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                let mut worry_level = (*monkeys[i].operation)(item);

                if is_very_worried {
                    worry_level %= common_divisor;
                } else {
                    worry_level /= 3;
                }

                let target_monkey = (*monkeys[i].test)(worry_level);

                monkeys[i].items_inspected += 1;
                monkeys[target_monkey].items.push(worry_level);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("data/day11_test");

    #[test]
    fn monkey_parsing_does_not_panic() {
        parse_monkeys(TEST_INPUT);
    }

    #[test]
    fn not_very_worried() {
        let mut monkeys = parse_monkeys(TEST_INPUT);
        do_monkey_business(&mut monkeys, 20, false);
        let items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
        assert_eq!(items_inspected, vec![101, 95, 7, 105]);
    }

    #[test]
    fn very_worried() {
        let mut monkeys = parse_monkeys(TEST_INPUT);
        do_monkey_business(&mut monkeys, 10_000, true);
        let items_inspected: Vec<_> = monkeys.iter().map(|m| m.items_inspected).collect();
        assert_eq!(items_inspected, vec![52_166, 47_830, 1_938, 52_013]);
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day11::new().part_1(), 57_838);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(Day11::new().part_2(), 15_050_382_231);
    }
}

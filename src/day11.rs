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
    op: F,
    test: F,
    divisor: usize,
}

impl Monkey {
    fn from_str(s: &str) -> Self {
        let lines: Vec<&str> = s
            .lines()
            .map(|line| line.split_once(':').unwrap().1)
            .collect();
        let items = lines[1]
            .trim()
            .split(", ")
            .map(|item_str| item_str.parse::<usize>().unwrap())
            .collect();
        let op_parts: Vec<&str> = lines[2]
            .split_once(" = ")
            .unwrap()
            .1
            .split_whitespace()
            .collect();
        let divisor = lines[3]
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .next()
            .unwrap();
        let true_target = lines[4]
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .next()
            .unwrap();
        let false_target = lines[5]
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .next()
            .unwrap();

        let op: F = match op_parts.as_slice() {
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

        Monkey {
            items,
            items_inspected: 0,
            op,
            test: Box::new(move |worry_level: usize| {
                if worry_level % divisor == 0 {
                    true_target
                } else {
                    false_target
                }
            }),
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
            for item in monkeys[i].items.clone() {
                let mut worry_level = (*monkeys[i].op)(item);

                if is_very_worried {
                    worry_level %= common_divisor;
                } else {
                    worry_level /= 3;
                }

                let target_monkey = (*monkeys[i].test)(worry_level);

                monkeys[i].items_inspected += 1;
                monkeys[target_monkey].items.push(worry_level);
            }
            monkeys[i].items.clear();
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

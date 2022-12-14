use crate::Solution;
use std::cmp::Ordering;

#[derive(Default)]
pub struct Day13 {}

impl Solution for Day13 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        include_str!("data/day13")
            .trim()
            .split("\n\n")
            .enumerate()
            .filter(|(_, pair)| {
                let (a, b) = pair.split_once('\n').unwrap();

                Value::from_str(a) < Value::from_str(b)
            })
            .map(|(index, _)| index + 1)
            .sum()
    }

    fn part_2(&self) -> Self::Result {
        let mut packets: Vec<(&str, Value)> = include_str!("data/day13")
            .split_whitespace()
            .map(|packet_str| (packet_str, Value::from_str(packet_str)))
            .collect();

        let divider_1: &str = "[[2]]";
        let divider_2: &str = "[[6]]";

        packets.push((divider_1, Value::from_str(divider_1)));
        packets.push((divider_2, Value::from_str(divider_2)));

        packets.sort_by(|(_, a), (_, b)| a.cmp(b));

        let first = packets.iter().position(|p| p.0 == divider_1).unwrap();
        let second = packets.iter().position(|p| p.0 == divider_2).unwrap();

        (first + 1) * (second + 1)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Int(usize),
    List(Vec<Value>),
}

impl Value {
    fn from_str(s: &str) -> Self {
        if s.starts_with('[') {
            // assume `s` is a valid list
            let mut list: Vec<Value> = vec![];

            // comma-separated `Value`s
            // e.g. "1,2,[3],[]"
            let mut inner = s.get(1..s.len() - 1).unwrap();

            // chomp each comma-separated inner value one at a time
            while !inner.is_empty() {
                if inner.starts_with('[') {
                    // the next value is a nested `Value::List`
                    let end_index = value_end_position(inner);
                    let (value_str, rest) = inner.split_at(end_index + 1);
                    list.push(Value::from_str(value_str));
                    inner = rest.strip_prefix(',').unwrap_or("");
                } else {
                    // the next value is a `Value::Int`
                    let (value_str, rest) = inner.split_once(',').unwrap_or((inner, ""));
                    list.push(Value::from_str(value_str));
                    inner = rest;
                }
            }

            Value::List(list)
        } else {
            Value::Int(s.parse().unwrap())
        }
    }
}

/// Assuming `s` starts with [, finds the index of the corresponding ]
fn value_end_position(s: &str) -> usize {
    let mut nest_level = 0;

    for (i, char) in s.chars().enumerate() {
        match char {
            '[' => nest_level += 1,
            ']' => nest_level -= 1,
            _ => continue,
        }

        if nest_level == 0 {
            return i;
        }
    }

    panic!("couldn't find matching ']'")
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        // the problem describes lexicographical comparison,
        // which `Ord` already uses, and `Vec` implements `Ord`,
        // so we can just use `cmp` here.
        // https://doc.rust-lang.org/std/cmp/trait.Ord.html#lexicographical-comparison
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Int(a), Self::List(b)) => vec![Value::Int(*a)].cmp(b),
            (Self::List(a), Self::Int(b)) => a.cmp(&vec![Value::Int(*b)]),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_parsing_works() {
        dbg!(Value::from_str("[[1],3,[2],[[]]]"));
    }

    fn assert_ord(is_ordered: bool, a: &str, b: &str) {
        assert_eq!(is_ordered, Value::from_str(a) < Value::from_str(b));
    }

    #[test]
    fn ordering_works() {
        assert_ord(true, "[1,1,3,1,1]", "[1,1,5,1,1]");

        assert_ord(true, "[[1],[2,3,4]]", "[[1],4]");

        assert_ord(false, "[9]", "[[8,7,6]]");

        assert_ord(true, "[[4,4],4,4]", "[[4,4],4,4,4]");

        assert_ord(false, "[7,7,7,7]", "[7,7,7]");

        assert_ord(true, "[]", "[3]");

        assert_ord(false, "[[[]]]", "[[]]");

        assert_ord(true, "[[]]", "[[[]]]");
    }

    #[test]
    fn part_1_works() {
        assert_eq!(Day13::new().part_1(), 5208);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(Day13::new().part_2(), 25792);
    }
}

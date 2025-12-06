use core::num;

use crate::Solution;

#[derive(Default)]
pub struct Day6 {}

impl Solution for Day6 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day6");
        grand_total(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day6");
        grand_total_cols(input)
    }
}

fn grand_total(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let rows: Vec<Vec<usize>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n_str| n_str.parse().expect("valid number"))
                .collect()
        })
        .collect();
    let ops: Vec<&str> = lines[lines.len() - 1].split_whitespace().collect();

    (0..rows[0].len())
        .into_iter()
        .map(|i| match ops[i] {
            "*" => rows
                .iter()
                .map(|row| row[i])
                .reduce(|acc, n| acc * n)
                .unwrap(),
            "+" => rows
                .iter()
                .map(|row| row[i])
                .reduce(|acc, n| acc + n)
                .unwrap(),
            op => panic!("invalid op: {}", op),
        })
        .sum()
}

fn grand_total_cols(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    let rows: Vec<Vec<char>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect();

    let mut ops_iter = lines[lines.len() - 1].split_whitespace().rev();
    let mut current_op = ops_iter.next().unwrap();

    let mut grand_total = 0;
    let mut current_total = None;

    for column in 0..rows[0].len() {
        let number_str: String = rows.iter().map(|row| row[column]).collect();

        match number_str.trim().parse::<usize>() {
            Ok(n) => {
                current_total = if let Some(total) = current_total {
                    match current_op {
                        "*" => Some(total * n),
                        "+" => Some(total + n),
                        op => panic!("invalid op: {}", op),
                    }
                } else {
                    Some(n)
                };
            },
            _ => {
                grand_total += current_total.expect("current_total should be nonzero");
                current_total = None;
                current_op = ops_iter.next().unwrap_or("ran out of ops");
            }
        }
    }

    grand_total + current_total.expect("current_total should be nonzero")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input =
"123 328  51 64 
  45 64  387 23 
   6 98  215 314
 *   +   *   +  ";

        assert_eq!(grand_total(input), 4277556);
    }

    #[test]
    fn part_2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(grand_total_cols(input), 3263827);
    }
}

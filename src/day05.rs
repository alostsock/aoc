use crate::Solution;

#[derive(Default)]
pub struct Day5 {}

impl Solution for Day5 {
    type P1 = String;
    type P2 = String;

    fn part_1(&self) -> Self::P1 {
        let (mut stacks, moves) = parse_stacks_and_moves(include_str!("data/day05"));
        for mv in moves {
            mv.perform(&mut stacks, false);
        }
        topmost_crates(&stacks)
    }

    fn part_2(&self) -> Self::P2 {
        let (mut stacks, moves) = parse_stacks_and_moves(include_str!("data/day05"));
        for mv in moves {
            mv.perform(&mut stacks, true);
        }
        topmost_crates(&stacks)
    }
}

type Stacks = [Vec<char>; 9];

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn perform(&self, stacks: &mut Stacks, maintain_order: bool) {
        let source = stacks.get_mut(self.from).unwrap();
        let index = source.len().saturating_sub(self.amount);
        let mut taken = source.split_off(index);

        if !maintain_order {
            taken.reverse();
        }

        let target = stacks.get_mut(self.to).unwrap();
        target.append(&mut taken);
    }
}

fn parse_stacks_and_moves(input: &str) -> (Stacks, Vec<Move>) {
    let (stacks_raw, moves_raw) = input.split_once("\n\n").unwrap();

    let mut stacks = Stacks::default();

    for line in stacks_raw.lines().rev().skip(1) {
        for (stack_index, crate_id) in line.chars().skip(1).step_by(4).enumerate() {
            if !crate_id.is_whitespace() {
                stacks[stack_index].push(crate_id);
            }
        }
    }

    let moves = moves_raw
        .lines()
        .map(|line| {
            let nums: Vec<_> = line.split(' ').flat_map(str::parse::<usize>).collect();

            Move {
                amount: nums[0],
                from: nums[1] - 1,
                to: nums[2] - 1,
            }
        })
        .collect();

    (stacks, moves)
}

fn topmost_crates(stacks: &Stacks) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn input_parsing_works() {
        let (stacks, ops) = parse_stacks_and_moves(TEST_INPUT);
        println!("{:?} {:?}", stacks, ops[0]);
    }

    #[test]
    fn moves_work() {
        let (mut stacks, moves) = parse_stacks_and_moves(TEST_INPUT);
        for mv in moves {
            mv.perform(&mut stacks, false);
        }
        assert_eq!(topmost_crates(&stacks), String::from("CMZ"));
    }

    #[test]
    fn moves_work_ordered() {
        let (mut stacks, moves) = parse_stacks_and_moves(TEST_INPUT);
        for mv in moves {
            mv.perform(&mut stacks, true);
        }
        assert_eq!(topmost_crates(&stacks), String::from("MCD"));
    }

    #[test]
    fn p1_solution() {
        assert_eq!(Day5::new().part_1(), String::from("TGWSMRBPN"));
    }

    #[test]
    fn p2_solution() {
        assert_eq!(Day5::new().part_2(), String::from("TZLTLWRNF"));
    }
}

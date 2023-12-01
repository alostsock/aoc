#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

use crate::Solution;

#[derive(Default)]
pub struct Day17 {}

impl Solution for Day17 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let stack = drop_shapes(include_str!("data/day17"));
        stack.len()
    }

    fn part_2(&self) -> Self::Result {
        todo!()
    }
}

fn drop_shapes(input: &str) -> Vec<Row> {
    let shape_set = [
        Shape::bar(),
        Shape::plus(),
        Shape::corner(),
        Shape::line(),
        Shape::square(),
    ];

    let mut shapes = shape_set.iter().cycle();
    let mut directions = input.trim().chars().cycle();

    let mut stack: Vec<Row> = vec![];

    let mut shape = shapes.next().unwrap().clone();
    let mut shapes_dropped: usize = 0;
    let mut height: isize = 3;

    while shapes_dropped < 2022 {
        let current_stack_index = safe_add(stack.len(), height);
        let current_stack_rows: Vec<&Row> = (0..shape.0.len())
            .rev()
            .map(|i| stack.get(current_stack_index + i).unwrap_or(&Row(0)))
            .collect();

        match directions.next().unwrap() {
            '>' => shape.shift_right(&current_stack_rows),
            '<' => shape.shift_left(&current_stack_rows),
            _ => panic!("invalid direction"),
        }

        height -= 1;

        let current_stack_index = safe_add(stack.len(), height);
        let current_stack_rows: Vec<&Row> = (0..shape.0.len())
            .rev()
            .map(|i| stack.get(current_stack_index + i).unwrap_or(&Row(0)))
            .collect();

        if height < 0
            && (stack.get(current_stack_index).is_none() || shape.hit_test(&current_stack_rows))
        {
            // the shape has settled. merge it into the stack, row by row
            for (i, row) in shape.0.iter().rev().enumerate() {
                if let Some(stack_row) =
                    stack.get_mut((current_stack_index as usize).saturating_add(i + 1))
                {
                    *stack_row = stack_row.merge(row);
                } else {
                    stack.push(*row);
                }
            }

            // println!(
            //     "{}",
            //     stack
            //         .iter()
            //         .rev()
            //         .map(|r| format!("{:#010b}", r.0))
            //         .collect::<Vec<_>>()
            //         .join("\n")
            // );

            shape = shapes.next().unwrap().clone();
            shapes_dropped += 1;
            println!("dropped {} shapes", shapes_dropped);
            height = 3;
        }
    }

    stack
}

#[derive(Clone, Copy)]
struct Row(u8);

impl Row {
    fn hit_test(&self, other: &Self) -> bool {
        (self.0 & other.0).count_ones() > 0
    }

    fn merge(&self, other: &Self) -> Self {
        assert!(self.0 & other.0 == 0);
        Self(self.0 | other.0)
    }
}

#[derive(Clone)]
struct Shape(Vec<Row>);

impl Shape {
    fn bar() -> Shape {
        Shape(vec![Row(0b0_0011110)])
    }

    fn plus() -> Shape {
        Shape(vec![Row(0b0_0001000), Row(0b0_0011100), Row(0b0_0001000)])
    }

    fn corner() -> Shape {
        Shape(vec![Row(0b0_0000100), Row(0b0_0000100), Row(0b0_0011100)])
    }

    fn line() -> Shape {
        Shape(vec![
            Row(0b0_0010000),
            Row(0b0_0010000),
            Row(0b0_0010000),
            Row(0b0_0010000),
        ])
    }

    fn square() -> Shape {
        Shape(vec![Row(0b0_0011000), Row(0b0_0011000)])
    }

    fn shift_left(&mut self, other_rows: &Vec<&Row>) {
        if self
            .0
            .iter()
            .zip(other_rows)
            .all(|(Row(a), Row(b))| a & 0b0_1000000 == 0 && a << 1 & b == 0)
        {
            for row in &mut self.0 {
                row.0 <<= 1;
            }
        }
    }

    fn shift_right(&mut self, other_rows: &Vec<&Row>) {
        if self
            .0
            .iter()
            .zip(other_rows)
            .all(|(Row(a), Row(b))| a & 0b0_0000001 == 0 && a >> 1 & b == 0)
        {
            for row in &mut self.0 {
                row.0 >>= 1;
            }
        }
    }

    fn hit_test(&self, other_rows: &Vec<&Row>) -> bool {
        self.0
            .iter()
            .zip(other_rows)
            .any(|(Row(a), Row(b))| a & b != 0)
    }
}

fn safe_add(a: usize, b: isize) -> usize {
    if b >= 0 {
        a.saturating_add(b as usize)
    } else {
        a.saturating_sub(b.abs() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn dropping_shapes_works() {
        let stack = drop_shapes(TEST_INPUT);
        assert_eq!(stack.len(), 3068);
    }

    // #[test]
    // fn part_1_works() {
    //     assert_eq!(Day17::new().part_1(), 2022);
    // }
}

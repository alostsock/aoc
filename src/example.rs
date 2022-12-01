use crate::Solution;

#[derive(Default)]
pub struct Example {}

impl Solution for Example {
    type P1 = String;
    type P2 = String;

    fn part_1(&self) -> Self::P1 {
        2022.to_string()
    }

    fn part_2(&self) -> Self::P2 {
        (2022 * 25).to_string()
    }
}

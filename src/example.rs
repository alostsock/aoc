use crate::Solution;

#[derive(Default)]
pub struct Example {}

impl Solution for Example {
    type Data = u32;
    type P1 = String;
    type P2 = String;

    fn data(&self) -> Self::Data {
        2022
    }

    fn part_1(&self, data: &Self::Data) -> Self::P1 {
        data.to_string()
    }

    fn part_2(&self, data: &Self::Data) -> Self::P2 {
        (data * 25).to_string()
    }
}

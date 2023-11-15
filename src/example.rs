use crate::Solution;

#[derive(Default)]
pub struct Example {}

impl Solution for Example {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        2023
    }

    fn part_2(&self) -> Self::Result {
        2023 * 25
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Example::new().part_1(), 2023);
    }
}

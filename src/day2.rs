use crate::Solution;

#[derive(Default)]
pub struct Day2 {}

impl Solution for Day2 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day2");
        sum_invalid_ids(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day2");
        sum_invalid_ids_multi(input)
    }
}

fn sum_invalid_ids(input: &str) -> usize {
    let mut invalid_id_sum = 0;

    for range_str in input.split(',') {
        let (start_str, end_str) = range_str.trim().split_once('-').expect("valid range");
        let start: usize = start_str.parse().unwrap();
        let end: usize = end_str.parse().unwrap();

        for id in start..=end {
          let id_str = id.to_string();
          if id_str.len() < 2 || id_str.len() % 2 != 0 {
            continue 
          }
          let (a, b) = id_str.split_at(id_str.len() / 2);
          let a: usize = a.parse().expect("valid number");
          let b: usize = b.parse().expect("valid number");
          if a == b {
            invalid_id_sum += id;
          }
        }
    }

    invalid_id_sum
}

fn sum_invalid_ids_multi(input: &str) -> usize {
    let mut invalid_id_sum = 0;

    for range_str in input.split(',') {
        let (start_str, end_str) = range_str.trim().split_once('-').expect("valid range");
        let start: usize = start_str.parse().unwrap();
        let end: usize = end_str.parse().unwrap();

        for id in start..=end {
          let id_str = id.to_string();
          let mut valid_lengths = (1..=(id_str.len() / 2)).filter(|l| {
            id_str.len() % l == 0
          });
          let chars: Vec<char> = id_str.chars().collect();
          if valid_lengths.any(|l| {
            let mut iter = chars.chunks(l).into_iter();
            let first = iter.next().expect("at least one number");
            iter.all(|n| n == first)
          }) {
            invalid_id_sum += id;
          }
        }
    }

    invalid_id_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(sum_invalid_ids(input), 1227775554);
    }

    #[test]
    fn part_2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(sum_invalid_ids_multi(input), 4174379265);
    }
}

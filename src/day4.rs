use crate::Solution;

#[derive(Default)]
pub struct Day4 {}

impl Solution for Day4 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day4");
        count_points(input)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day4");
        count_cards(input)
    }
}

struct Numbers(u128);

impl Numbers {
    fn from_str(numbers_str: &str) -> Self {
        let numbers = numbers_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap());
        let bits: u128 = numbers.fold(0, |bits, number: u8| bits | (1 << number));
        Self(bits)
    }

    fn compare_numbers(&self, other: &Self) -> u32 {
        (self.0 & other.0).count_ones()
    }
}

struct Card {
    winning_numbers: Numbers,
    drawn_numbers: Numbers,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let (_card_label, rest) = line.split_once(':').unwrap();
        let (winning_numbers, drawn_numbers) = rest.split_once('|').unwrap();
        let winning_numbers = Numbers::from_str(winning_numbers);
        let drawn_numbers = Numbers::from_str(drawn_numbers);

        Self {
            winning_numbers,
            drawn_numbers,
        }
    }

    fn matching_numbers(&self) -> u32 {
        self.winning_numbers.compare_numbers(&self.drawn_numbers)
    }

    fn points(&self) -> usize {
        let matches = self.matching_numbers();
        if matches > 0 {
            2_usize.pow(matches - 1)
        } else {
            0
        }
    }
}

fn count_points(input: &str) -> usize {
    input
        .lines()
        .map(|line| Card::from_line(line).points())
        .sum()
}

fn count_cards(input: &str) -> usize {
    let cards: Vec<_> = input.lines().map(Card::from_line).collect();
    let mut card_quantities = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let matches = card.matching_numbers() as usize;
        let current_card_quantity = card_quantities[index];
        for new_card_index in (index + 1)..=(index + matches) {
            card_quantities[new_card_index] += current_card_quantity;
        }
    }

    card_quantities.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1() {
        assert_eq!(count_points(INPUT), 13);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_cards(INPUT), 30);
    }
}

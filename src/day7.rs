use crate::Solution;

#[derive(Default)]
pub struct Day7 {}

impl Solution for Day7 {
    type Result = usize;

    fn part_1(&self) -> Self::Result {
        let input = include_str!("data/day7");
        determine_total_winnings(input, false)
    }

    fn part_2(&self) -> Self::Result {
        let input = include_str!("data/day7");
        determine_total_winnings(input, true)
    }
}

enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    fn from_card_strengths(card_strengths: &Vec<usize>, jokers_enabled: bool) -> Self {
        let mut tally = [0; 14];
        let mut jokers = 0;
        for &strength in card_strengths {
            if jokers_enabled && strength == 0 {
                jokers += 1;
            }
            tally[strength] += 1;
        }
        tally.sort_unstable();
        tally.reverse();
        if jokers_enabled {
            tally[0] += jokers;
        }
        match tally {
            [5, ..] => Self::FiveOfAKind,
            [4, 1, 1, 1, ..] => Self::ThreeOfAKind, // joker edge case "AJJ23"
            [4, ..] => Self::FourOfAKind,
            [3, 2, ..] => Self::FullHouse,
            [3, 1, ..] => Self::ThreeOfAKind,
            [2, 2, ..] => Self::TwoPair,
            [2, ..] => Self::OnePair,
            _ if tally[0] > 5 => Self::FiveOfAKind, // joker edge cases "AJJJJ" and "AAJJJ"
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug)]
struct Hand {
    card_strengths: Vec<usize>,
    hand_kind_strength: usize,
    bid: usize,
}

impl Hand {
    fn from_str(s: &str, jokers_enabled: bool) -> Self {
        let (labels, bid_str) = s.split_once(' ').unwrap();
        let card_strengths = labels
            .chars()
            .map(|ch| match ch {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => {
                    if !jokers_enabled {
                        10
                    } else {
                        0
                    }
                }
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                _ => panic!("invalid label: {ch}"),
            })
            .collect();

        let hand_kind_strength =
            match HandKind::from_card_strengths(&card_strengths, jokers_enabled) {
                HandKind::FiveOfAKind => 6,
                HandKind::FourOfAKind => 5,
                HandKind::FullHouse => 4,
                HandKind::ThreeOfAKind => 3,
                HandKind::TwoPair => 2,
                HandKind::OnePair => 1,
                HandKind::HighCard => 0,
            };

        let bid = bid_str.parse().unwrap();

        Self {
            card_strengths,
            hand_kind_strength,
            bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_kind_strength.eq(&other.hand_kind_strength) {
            self.card_strengths.eq(&other.card_strengths)
        } else {
            false
        }
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_kind_strength.eq(&other.hand_kind_strength) {
            for i in 0..5 {
                if self.card_strengths[i].ne(&other.card_strengths[i]) {
                    return self.card_strengths[i].partial_cmp(&other.card_strengths[i]);
                }
            }
            Some(std::cmp::Ordering::Equal)
        } else {
            self.hand_kind_strength
                .partial_cmp(&other.hand_kind_strength)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn determine_total_winnings(input: &str, jokers_enabled: bool) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::from_str(line, jokers_enabled))
        .collect();
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let rank = index + 1;
            rank * hand.bid
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        assert_eq!(determine_total_winnings(INPUT, false), 6440);
        assert_eq!(Day7::new().part_1(), 248422077);
    }

    #[test]
    fn part_2() {
        assert_eq!(determine_total_winnings(INPUT, true), 5905);
        assert_eq!(Day7::new().part_2(), 249817836);
    }

    #[test]
    fn wildcard_ordering() {
        assert!(Hand::from_str("AJJ22 1", true) > Hand::from_str("AJJ23 1", true));
        assert!(Hand::from_str("AAAJJ 1", true) > Hand::from_str("AAJJJ 1", true));
        assert!(Hand::from_str("AAJJJ 1", true) > Hand::from_str("AJJJJ 1", true));
    }
}

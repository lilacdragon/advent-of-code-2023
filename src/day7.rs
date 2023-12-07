use std::collections::HashMap;

use itertools::Itertools;

use crate::DaySolution;

pub struct Day7;

impl DaySolution for Day7 {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let (hand, bid) = l.split_once(' ').unwrap();
                let bid = bid.parse::<u64>().unwrap();
                let hand_type = parse_hand_type(hand);
                Hand {
                    hand: parse_cards(hand),
                    hand_type,
                    bid,
                }
            })
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum::<u64>()
            .to_string()
    }
    fn part2(input: &str) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn parse_cards(hand: &str) -> [Card; 5] {
    let mut cards = [Card::Two; 5];
    for (i, c) in hand.chars().enumerate() {
        cards[i] = match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
    cards
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    hand: [Card; 5],
    hand_type: HandType,
    bid: u64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => Some(self.hand.cmp(&other.hand)),
            o => Some(o),
        }
    }
}

fn parse_hand_type(hand: &str) -> HandType {
    let mut seen = HashMap::new();
    for c in hand.chars() {
        *seen.entry(c).or_insert(0) += 1;
    }
    match seen
        .into_values()
        .sorted()
        .rev()
        .take(2)
        .next_tuple()
        .unwrap_or((5, 0)) // Edge case, since five of a kind will only have one entry in map
    {
        (5, 0) => HandType::Five,
        (4, 1) => HandType::Four,
        (3, 2) => HandType::FullHouse,
        (3, 1) => HandType::Three,
        (2, 2) => HandType::TwoPair,
        (2, 1) => HandType::Pair,
        (1, 1) => HandType::High,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day7::part1(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "6440".to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day7::part2(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "".to_string()
        );
    }
}

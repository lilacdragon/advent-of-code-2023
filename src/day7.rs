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
                let hand_type = parse_hand_type(hand, false).0;
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
        input
            .lines()
            .map(|l| {
                let (hand, bid) = l.split_once(' ').unwrap();
                let bid = bid.parse::<u64>().unwrap();
                let (hand_type, jokers) = parse_hand_type(hand, true);
                Hand {
                    hand: parse_cards(hand)
                        .into_iter()
                        .map(|c| match c {
                            Card::Jack => Card::Joker,
                            c => c,
                        })
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                    hand_type: joker_upgrade(hand_type, jokers),
                    bid,
                }
            })
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank as u64 + 1) * hand.bid)
            .sum::<u64>()
            .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
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

fn parse_hand_type(hand: &str, jokers: bool) -> (HandType, u64) {
    let mut seen = HashMap::new();
    for c in hand.chars() {
        *seen.entry(c).or_insert(0) += 1;
    }
    let joker_count = seen.get(&'J').copied().unwrap_or(0);
    if jokers {
        seen.remove(&'J');
    }
    let hand_type = match seen
        .values()
        .cloned()
        .sorted()
        .rev()
        .take(2)
        .next_tuple()
        .unwrap_or((*seen.values().max().unwrap_or( & 5), 0))
    {
        (5, 0) => HandType::Five,
        (4, _) => HandType::Four,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::Three,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::Pair,
        (1, _) => HandType::High,
        _ => unreachable!(),
    };

    (hand_type, joker_count)
}

fn joker_upgrade(hand: HandType, jokers: u64) -> HandType {
    match (hand, jokers) {
        (HandType::High, 1) => HandType::Pair,
        (HandType::High, 2) => HandType::Three,
        (HandType::High, 3) => HandType::Four,
        (HandType::High, 4) => HandType::Five,
        (HandType::Pair, 1) => HandType::Three,
        (HandType::Pair, 2) => HandType::Four,
        (HandType::Pair, 3) => HandType::Five,
        (HandType::TwoPair, 1) => HandType::FullHouse,
        (HandType::Three, 1) => HandType::Four,
        (HandType::Three, 2) => HandType::Five,
        (HandType::Four, 1) => HandType::Five,
        (hand, _) => hand,
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
            "5905".to_string()
        );
    }
}

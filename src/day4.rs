use std::collections::HashSet;

use crate::DaySolution;

pub struct Day4;

impl DaySolution for Day4 {
    fn star_one(input: &str) -> String {
        input
            .lines()
            .map(winning_numbers_count)
            .filter_map(|count| count.checked_sub(1))
            .map(|count| 2u32.pow(count as u32))
            .sum::<u32>()
            .to_string()
    }

    fn star_two(input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();
        let mut card_count = vec![1; lines.len()];

        for (card_number, line) in lines.iter().enumerate() {
            let winning_count = winning_numbers_count(line);
            for next in card_number..card_number + winning_count {
                card_count[next + 1] += card_count[card_number];
            }
        }

        card_count.iter().sum::<u32>().to_string()
    }
}

fn parse_number_set(numbers: &str) -> HashSet<usize> {
    numbers
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_line(line: &str) -> (HashSet<usize>, HashSet<usize>) {
    let (_, numbers) = line.split_once(": ").unwrap();
    let (winning, mine) = numbers.split_once(" | ").unwrap();
    (parse_number_set(winning), parse_number_set(mine))
}

fn winning_numbers_count(line: &str) -> usize {
    let (winning, mine) = parse_line(line);
    winning.intersection(&mine).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day4::star_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "13"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            Day4::star_two(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "30"
        );
    }
}

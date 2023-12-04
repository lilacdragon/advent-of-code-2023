use std::collections::HashSet;

use crate::DaySolution;

pub struct Day4;

impl DaySolution for Day4 {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let numbers = line.split_once(": ").unwrap().1;
                let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();
                let winning_numbers = parse_numbers(winning_numbers);
                let my_numbers = parse_numbers(my_numbers);
                my_numbers.intersection(&winning_numbers).count()
            })
            .filter_map(|count| count.checked_sub(1))
            .map(|count| 2u32.pow(count as u32))
            .sum::<u32>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        todo!()
    }
}

fn parse_numbers(numbers: &str) -> HashSet<usize> {
    numbers
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day4::part1(
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
    fn test_part2() {}
}
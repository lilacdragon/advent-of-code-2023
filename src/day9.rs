use crate::DaySolution;
use itertools::Itertools;

pub struct Day9;

impl DaySolution for Day9 {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let numbers = parse_line(l);
                predict_next(numbers)
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        "".to_string()
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn predict_next(history: Vec<i64>) -> i64 {
    let derivatives = calc_derivatives(&history);
    let mut next = history.last().unwrap().clone();
    for d in derivatives.iter() {
        next += d.last().unwrap();
    }
    next
}

fn calc_derivatives(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    fn recursive_derive(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
        if sequence.iter().all(|&x| x == 0) {
            return Vec::new();
        }
        let mut derivatives = Vec::new();
        for (a, b) in sequence.iter().tuple_windows() {
            derivatives.push(b - a);
        }
        let mut output = Vec::new();
        let nexts = recursive_derive(&derivatives);
        output.push(derivatives);
        output.extend(nexts);
        output
    }
    recursive_derive(sequence)
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day9::part1(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "114"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day9::part2(""), "");
    }
}

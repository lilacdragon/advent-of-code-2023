use crate::DaySolution;
use itertools::Itertools;
use std::ops::Index;

pub struct Day9;

impl DaySolution for Day9 {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let numbers = parse_line(l);
                predict(numbers, Prediction::Next)
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let numbers = parse_line(l);
                predict(numbers, Prediction::Previous)
            })
            .sum::<i64>()
            .to_string()
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Prediction {
    Next,
    Previous,
}

impl<T> Index<Prediction> for Vec<T> {
    type Output = T;

    fn index(&self, index: Prediction) -> &Self::Output {
        match index {
            Prediction::Next => self.last().unwrap(),
            Prediction::Previous => self.first().unwrap(),
        }
    }
}

impl Prediction {
    fn sign(&self) -> i64 {
        match self {
            Prediction::Next => 1,
            Prediction::Previous => -1,
        }
    }
}

fn predict(history: Vec<i64>, direction: Prediction) -> i64 {
    let derivatives = calc_derivatives(&history);
    let last = history[direction];
    let offset = derivatives
        .into_iter()
        .rfold(0, |acc, x| acc * direction.sign() + x[direction]);
    last + offset * direction.sign()
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
        assert_eq!(
            Day9::part2(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "2"
        );
    }
}

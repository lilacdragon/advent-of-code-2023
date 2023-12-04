use std::ops::Add;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::DaySolution;

pub struct Day2;

impl DaySolution for Day2 {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|line| game(line).unwrap().1)
            .filter(|(_, sets)| {
                sets.iter()
                    .all(|set| set.blue <= 14 && set.red <= 12 && set.green <= 13)
            })
            .map(|(n, _)| n)
            .sum::<usize>()
            .to_string()
    }
    fn part2(input: &str) -> String {
        input
            .lines()
            .map(|line| game(line).unwrap().1 .1)
            .map(|sets| {
                sets.into_iter()
                    .reduce(|a, b| a.compute_minimums(&b))
                    .unwrap()
            })
            .map(|set| set.blue * set.red * set.green)
            .sum::<usize>()
            .to_string()
    }
}

fn game_number(input: &str) -> IResult<&str, usize> {
    let (input, (_, number, _)) = tuple((tag("Game "), digit1, tag(": ")))(input)?;
    Ok((input, number.parse().unwrap()))
}

#[derive(Debug, Default, PartialEq, Eq)]
struct RevealSet {
    blue: usize,
    red: usize,
    green: usize,
}

impl RevealSet {
    fn compute_minimums(&self, other: &Self) -> Self {
        Self {
            blue: self.blue.max(other.blue),
            red: self.red.max(other.red),
            green: self.green.max(other.green),
        }
    }
}

impl Add for RevealSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            blue: self.blue + rhs.blue,
            red: self.red + rhs.red,
            green: self.green + rhs.green,
        }
    }
}

fn color_count(input: &str) -> IResult<&str, RevealSet> {
    let (input, (number, color)) = separated_pair(digit1, tag(" "), alpha1)(input)?;
    let mut set = RevealSet::default();
    match color {
        "blue" => set.blue = number.parse().unwrap(),
        "red" => set.red = number.parse().unwrap(),
        "green" => set.green = number.parse().unwrap(),
        _ => unreachable!(),
    }
    Ok((input, set))
}

fn reveal_set(input: &str) -> IResult<&str, RevealSet> {
    let (input, counts) = separated_list1(tag(", "), color_count)(input)?;
    let set = counts.into_iter().reduce(RevealSet::add).unwrap();
    Ok((input, set))
}

fn game(input: &str) -> IResult<&str, (usize, Vec<RevealSet>)> {
    let (input, (number, sets)) =
        tuple((game_number, separated_list1(tag("; "), reveal_set)))(input)?;
    Ok((input, (number, sets)))
}

#[cfg(test)]
mod tests {
    use super::{Day2, RevealSet};
    use crate::DaySolution;

    #[test]
    fn part_1() {
        assert_eq!(
            Day2::part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8"
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Day2::part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "2286"
        );
    }

    #[test]
    fn test_game_number() {
        assert_eq!(
            super::game_number("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1))
        );
    }

    #[test]
    fn test_color_count() {
        assert_eq!(
            super::color_count("3 blue"),
            Ok((
                "",
                RevealSet {
                    blue: 3,
                    red: 0,
                    green: 0
                }
            ))
        );
        assert_eq!(
            super::color_count("2 green, 6 blue"),
            Ok((
                ", 6 blue",
                super::RevealSet {
                    blue: 0,
                    red: 0,
                    green: 2
                }
            ))
        );
        assert_eq!(
            super::color_count("4 red, 13 green"),
            Ok((
                ", 13 green",
                super::RevealSet {
                    blue: 0,
                    red: 4,
                    green: 0
                }
            ))
        );
    }

    #[test]
    fn test_reveal_set() {
        assert_eq!(
            super::reveal_set("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok((
                "; 1 red, 2 green, 6 blue; 2 green",
                RevealSet {
                    blue: 3,
                    red: 4,
                    green: 0
                }
            ))
        );
        assert_eq!(
            super::reveal_set("1 red, 2 green, 6 blue; 2 green"),
            Ok((
                "; 2 green",
                RevealSet {
                    blue: 6,
                    red: 1,
                    green: 2
                }
            ))
        );
    }

    #[test]
    fn test_game() {
        assert_eq!(
            super::game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok((
                "",
                (
                    1,
                    vec![
                        RevealSet {
                            blue: 3,
                            red: 4,
                            green: 0
                        },
                        RevealSet {
                            blue: 6,
                            red: 1,
                            green: 2
                        },
                        RevealSet {
                            blue: 0,
                            red: 0,
                            green: 2
                        }
                    ]
                )
            ))
        );
    }
}

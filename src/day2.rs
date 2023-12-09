use crate::DaySolution;

pub struct Day2;

impl DaySolution for Day2 {
    fn star_one(input: &str) -> String {
        input
            .lines()
            .map(|line| parse_line(line))
            .filter(|(_, counts)| counts.blue <= 14 && counts.red <= 12 && counts.green <= 13)
            .map(|(n, _)| n)
            .sum::<usize>()
            .to_string()
    }
    fn star_two(input: &str) -> String {
        input
            .lines()
            .map(|line| parse_line(line).1)
            .map(|counts| counts.blue * counts.red * counts.green)
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct ColorCounts {
    blue: usize,
    red: usize,
    green: usize,
}

fn parse_line_counts(counts: Vec<&str>) -> ColorCounts {
    use std::cmp::max;

    let mut color_counts = ColorCounts::default();
    for count in counts {
        let (number, color) = count.split_once(' ').unwrap();
        match color {
            "blue" => color_counts.blue = max(number.parse().unwrap(), color_counts.blue),
            "red" => color_counts.red = max(number.parse().unwrap(), color_counts.red),
            "green" => color_counts.green = max(number.parse().unwrap(), color_counts.green),
            _ => unreachable!(),
        }
    }
    color_counts
}

fn parse_line(line: &str) -> (usize, ColorCounts) {
    let (number, sets) = line.split_once(": ").unwrap();
    let number = number.split_once(' ').unwrap().1.parse().unwrap();
    let sets = parse_line_counts(sets.split("; ").map(|s| s.split(", ")).flatten().collect());

    (number, sets)
}

#[cfg(test)]
mod tests {
    use super::{ColorCounts, Day2};
    use crate::DaySolution;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day2::star_one(
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
    fn test_star_two() {
        assert_eq!(
            Day2::star_two(
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
    fn test_parse_line_counts() {
        assert_eq!(
            super::parse_line_counts(vec!["3 blue", "4 red"]),
            ColorCounts {
                blue: 3,
                red: 4,
                green: 0
            }
        );
        assert_eq!(
            super::parse_line_counts(vec!["1 red", "2 green", "6 blue"]),
            ColorCounts {
                blue: 6,
                red: 1,
                green: 2
            }
        );
        assert_eq!(
            super::parse_line_counts(vec!["2 green"]),
            ColorCounts {
                blue: 0,
                red: 0,
                green: 2
            }
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            super::parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (
                1,
                ColorCounts {
                    blue: 6,
                    red: 4,
                    green: 2
                }
            )
        );
    }
}

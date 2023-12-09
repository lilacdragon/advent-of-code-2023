use std::collections::{HashMap, HashSet};

use crate::DaySolution;

const AROUND: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub struct Day3;

impl DaySolution for Day3 {
    fn star_one(input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let mut numbers: Vec<usize> = Vec::new();
        for row in 0..grid.len() {
            let mut current_number = String::new();
            let mut current_number_valid = false;
            for col in 0..grid[row].len() {
                let c = grid[row][col];
                match c {
                    '0'..='9' => {
                        current_number.push(c);
                        current_number_valid = current_number_valid
                            || AROUND.iter().any(|(row_offset, col_offset)| {
                                let row = row as isize + row_offset;
                                let col = col as isize + col_offset;
                                grid.get(row as usize)
                                    .and_then(|r| r.get(col as usize))
                                    .map(|val| valid_symbol(*val))
                                    .unwrap_or(false)
                            });
                    }
                    _ => {
                        if current_number_valid {
                            numbers.push(current_number.parse::<usize>().unwrap());
                        }
                        current_number = String::new();
                        current_number_valid = false;
                    }
                }
            }
            if current_number_valid {
                numbers.push(current_number.parse::<usize>().unwrap());
            }
        }

        numbers.iter().sum::<usize>().to_string()
    }

    fn star_two(input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let mut gears = HashMap::new();
        for row in 0..grid.len() {
            let mut current_number = String::new();
            let mut current_gears = HashSet::new();
            for col in 0..grid[row].len() {
                let c = grid[row][col];
                match c {
                    '0'..='9' => {
                        current_number.push(c);
                        for (row_offset, col_offset) in AROUND.iter() {
                            let row = row as isize + row_offset;
                            let col = col as isize + col_offset;
                            if grid
                                .get(row as usize)
                                .and_then(|r| r.get(col as usize))
                                .map(|val| *val == '*')
                                .unwrap_or(false)
                            {
                                current_gears.insert((row as usize, col as usize));
                            }
                        }
                    }
                    _ => {
                        for gear in current_gears {
                            gears
                                .entry(gear)
                                .or_insert(Vec::new())
                                .push(current_number.parse::<usize>().unwrap());
                        }
                        current_number = String::new();
                        current_gears = HashSet::new();
                    }
                }
            }
            for gear in current_gears {
                gears
                    .entry(gear)
                    .or_insert(Vec::new())
                    .push(current_number.parse::<usize>().unwrap());
            }
        }

        gears
            .values()
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers[0] * numbers[1])
            .sum::<usize>()
            .to_string()
    }
}

fn valid_symbol(c: char) -> bool {
    match c {
        '0'..='9' => false,
        '.' => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day3::star_one(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "4361"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            Day3::star_two(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "467835"
        );
    }
}

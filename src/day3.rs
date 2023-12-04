use std::collections::{HashMap, HashSet};

use crate::DaySolution;

pub struct Day3;

impl DaySolution for Day3 {
    fn part1(input: &str) -> String {
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
                        // Top left
                        if row > 0 && col > 0 && valid_symbol(grid[row - 1][col - 1]) {
                            current_number_valid = true;
                        }
                        // Left
                        if col > 0 && valid_symbol(grid[row][col - 1]) {
                            current_number_valid = true;
                        }
                        // Bottom left
                        if row < grid.len() - 1 && col > 0 && valid_symbol(grid[row + 1][col - 1]) {
                            current_number_valid = true;
                        }
                        // Above
                        if row > 0 && valid_symbol(grid[row - 1][col]) {
                            current_number_valid = true;
                        }
                        // Below
                        if row < grid.len() - 1 && valid_symbol(grid[row + 1][col]) {
                            current_number_valid = true;
                        }
                        // Top right
                        if row > 0
                            && col < grid[row].len() - 1
                            && valid_symbol(grid[row - 1][col + 1])
                        {
                            current_number_valid = true;
                        }
                        // Right
                        if col < grid[row].len() - 1 && valid_symbol(grid[row][col + 1]) {
                            current_number_valid = true;
                        }
                        // Bottom right
                        if row < grid.len() - 1
                            && col < grid[row].len() - 1
                            && valid_symbol(grid[row + 1][col + 1])
                        {
                            current_number_valid = true;
                        }
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

    fn part2(input: &str) -> String {
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
                        // Top left
                        if row > 0 && col > 0 && grid[row - 1][col - 1] == '*' {
                            current_gears.insert((row - 1, col - 1));
                        }
                        // Left
                        if col > 0 && grid[row][col - 1] == '*' {
                            current_gears.insert((row, col - 1));
                        }
                        // Bottom left
                        if row < grid.len() - 1 && col > 0 && grid[row + 1][col - 1] == '*' {
                            current_gears.insert((row + 1, col - 1));
                        }
                        // Above
                        if row > 0 && grid[row - 1][col] == '*' {
                            current_gears.insert((row - 1, col));
                        }
                        // Below
                        if row < grid.len() - 1 && grid[row + 1][col] == '*' {
                            current_gears.insert((row + 1, col));
                        }
                        // Top right
                        if row > 0 && col < grid[row].len() - 1 && grid[row - 1][col + 1] == '*' {
                            current_gears.insert((row - 1, col + 1));
                        }
                        // Right
                        if col < grid[row].len() - 1 && grid[row][col + 1] == '*' {
                            current_gears.insert((row, col + 1));
                        }
                        // Bottom right
                        if row < grid.len() - 1
                            && col < grid[row].len() - 1
                            && grid[row + 1][col + 1] == '*'
                        {
                            current_gears.insert((row + 1, col + 1));
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

        let mut total_gear_ratios = 0;
        for numbers in gears.values() {
            if numbers.len() != 2 {
                continue;
            }
            let ratio = numbers[0] * numbers[1];
            total_gear_ratios += ratio;
        }

        total_gear_ratios.to_string()
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
    fn test_part1() {
        assert_eq!(
            Day3::part1(
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
    fn test_part2() {
        assert_eq!(
            Day3::part2(
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

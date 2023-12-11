use crate::DaySolution;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub struct Day11;

impl DaySolution for Day11 {
    fn star_one(input: &str) -> String {
        let mut image = Image::from(input);
        image.calc_expansion(2);
        image.solve_shortest_distances().to_string()
    }

    fn star_two(input: &str) -> String {
        let mut image = Image::from(input);
        image.calc_expansion(1_000_000);
        image.solve_shortest_distances().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Empty,
    Galaxy,
}

struct Image {
    pixels: Vec<Vec<Pixel>>,
    empty_rows: HashSet<usize>,
    empty_columns: HashSet<usize>,
    expansion_factor: usize,
}

impl From<&str> for Image {
    fn from(input: &str) -> Self {
        Image {
            pixels: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Pixel::Empty,
                            '#' => Pixel::Galaxy,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
            empty_rows: HashSet::new(),
            empty_columns: HashSet::new(),
            expansion_factor: 0,
        }
    }
}

impl Image {
    fn calc_expansion(&mut self, factor: usize) {
        let image = &self.pixels;

        let empty_rows = (0..image.len())
            .filter(|row| image[*row].iter().all(|p| *p == Pixel::Empty))
            .rev()
            .collect();

        let empty_columns = (0..image[0].len())
            .filter(|column| image.iter().all(|row| row[*column] == Pixel::Empty))
            .rev()
            .collect();

        self.empty_rows = empty_rows;
        self.empty_columns = empty_columns;
        self.expansion_factor = factor;
    }

    fn solve_shortest_distances(&self) -> usize {
        let galaxy_positions = self
            .pixels
            .iter()
            .enumerate()
            .map(|(row, pixels)| {
                pixels
                    .into_iter()
                    .enumerate()
                    .map(move |(column, pixel)| ((row, column), pixel))
            })
            .flatten()
            .filter(|(_, pixel)| **pixel == Pixel::Galaxy)
            .map(|(position, _)| position)
            .map(|(row, column)| (row, column));
        galaxy_positions
            .combinations_with_replacement(2)
            .map(|elements| {
                let a = elements[0];
                let b = elements[1];
                let empty_rows = fixed_range(a.0, b.0)
                    .filter(|row| self.empty_rows.contains(row))
                    .count();
                let row_expansion = empty_rows * self.expansion_factor;
                let empty_columns = fixed_range(a.1, b.1)
                    .filter(|column| self.empty_columns.contains(column))
                    .count();
                let column_expansion = empty_columns * self.expansion_factor;
                (a.0 as isize - b.0 as isize).abs() as usize
                    + (a.1 as isize - b.1 as isize).abs() as usize
                    + row_expansion
                    + column_expansion
                    - empty_rows
                    - empty_columns
            })
            .sum()
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for pixel in row {
                match pixel {
                    Pixel::Empty => write!(f, ".")?,
                    Pixel::Galaxy => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn fixed_range(a: usize, b: usize) -> Range<usize> {
    if a < b {
        a..b
    } else {
        b..a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day11::star_one(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            "374".to_string()
        );
    }

    #[test]
    fn test_star_two() {}

    #[test]
    fn test_expansion() {
        let mut image = Image::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        image.calc_expansion(2);
        assert_eq!(image.solve_shortest_distances(), 374);
        image.calc_expansion(10);
        assert_eq!(image.solve_shortest_distances(), 1030);
        image.calc_expansion(100);
        assert_eq!(image.solve_shortest_distances(), 8410);
    }
}

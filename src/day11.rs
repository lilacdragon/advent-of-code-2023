use crate::DaySolution;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub struct Day11;

impl DaySolution for Day11 {
    fn star_one(input: &str) -> String {
        let mut image = Image::from(input);
        image.calc_expansion(1);
        image.solve_shortest_distances().to_string()
    }

    fn star_two(input: &str) -> String {
        "".to_string()
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

    fn solve_shortest_distances(self) -> isize {
        let galaxy_positions = self
            .pixels
            .into_iter()
            .enumerate()
            .map(|(row, pixels)| {
                pixels
                    .into_iter()
                    .enumerate()
                    .map(move |(column, pixel)| ((row, column), pixel))
            })
            .flatten()
            .filter(|(_, pixel)| *pixel == Pixel::Galaxy)
            .map(|(position, _)| position);
        galaxy_positions
            .combinations_with_replacement(2)
            .map(|elements| {
                let a = elements[0];
                let b = elements[1];
                let row_expansion = fixed_range(a.0, b.0)
                    .filter(|row| self.empty_rows.contains(row))
                    .count()
                    * self.expansion_factor;
                let column_expansion = fixed_range(a.1, b.1)
                    .filter(|column| self.empty_columns.contains(column))
                    .count()
                    * self.expansion_factor;
                dbg!(a, b, row_expansion, column_expansion);
                (a.0 as isize - b.0 as isize).abs()
                    + (a.1 as isize - b.1 as isize).abs()
                    + row_expansion as isize
                    + column_expansion as isize
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
}

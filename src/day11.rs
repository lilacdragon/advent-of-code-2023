use crate::DaySolution;
use itertools::Itertools;
use std::fmt::{Debug, Formatter};

pub struct Day11;

impl DaySolution for Day11 {
    fn star_one(input: &str) -> String {
        let mut image = Image::from(input);
        image = image.expand();
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
        }
    }
}

impl Image {
    fn expand(self) -> Self {
        let mut image = self.pixels;

        let empty_rows: Vec<usize> = (0..image.len())
            .filter(|row| image[*row].iter().all(|p| *p == Pixel::Empty))
            .rev()
            .collect();
        for row in empty_rows {
            image.insert(row, vec![Pixel::Empty; image[row].len()]);
        }

        let empty_columns: Vec<usize> = (0..image[0].len())
            .filter(|column| image.iter().all(|row| row[*column] == Pixel::Empty))
            .rev()
            .collect();
        for row in &mut image {
            for column in &empty_columns {
                row.insert(*column, Pixel::Empty);
            }
        }

        Image { pixels: image }
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
                (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()
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
    fn test_star_two() {
        assert_eq!(Day11::star_two(""), "".to_string());
    }

    #[test]
    fn test_image_expansion() {
        let image = Image::from(
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
        assert_eq!(
            format!("{:?}", image.expand()).trim(),
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
        );
    }
}

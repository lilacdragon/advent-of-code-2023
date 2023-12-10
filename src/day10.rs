use crate::DaySolution;
use std::ops::{Add, Index};

pub struct Day10;

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl DaySolution for Day10 {
    fn star_one(input: &str) -> String {
        let tiles = Tiles::from(input);

        let (mut current_position, mut last_direction) = DIRECTIONS
            .iter()
            .find_map(|direction| {
                let next_position = tiles.start + direction.get_offset();
                match tiles.get(next_position) {
                    Some(pipe) if pipe.connects_in_direction(direction.opposite()) => {
                        Some((next_position, direction.opposite()))
                    }
                    _ => None,
                }
            })
            .unwrap();

        let mut steps = 1;
        while tiles.get(current_position).unwrap() != Pipe::Start {
            (current_position, last_direction) = DIRECTIONS
                .into_iter()
                .filter(|&d| d != last_direction)
                .filter(|&d| {
                    tiles
                        .get(current_position)
                        .unwrap()
                        .connects_in_direction(d)
                })
                .find_map(|d| {
                    let next_position = current_position + d.get_offset();
                    if let Some(_) = tiles.get(next_position) {
                        Some((next_position, d.opposite()))
                    } else {
                        None
                    }
                })
                .unwrap();
            steps += 1;
        }

        (steps / 2).to_string()
    }

    fn star_two(input: &str) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    None,
    Start,
}

impl Pipe {
    fn connects_in_direction(&self, direction: Direction) -> bool {
        match (self, direction) {
            (Pipe::Vertical, Direction::North) => true,
            (Pipe::Vertical, Direction::South) => true,
            (Pipe::Horizontal, Direction::East) => true,
            (Pipe::Horizontal, Direction::West) => true,
            (Pipe::NorthEast, Direction::North) => true,
            (Pipe::NorthEast, Direction::East) => true,
            (Pipe::NorthWest, Direction::North) => true,
            (Pipe::NorthWest, Direction::West) => true,
            (Pipe::SouthWest, Direction::South) => true,
            (Pipe::SouthWest, Direction::West) => true,
            (Pipe::SouthEast, Direction::South) => true,
            (Pipe::SouthEast, Direction::East) => true,
            _ => false,
        }
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::None,
            'S' => Pipe::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position(isize, isize);

impl Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        Position(self.0 + x, self.1 + y)
    }
}

#[derive(Debug)]
struct Tiles {
    tiles: Vec<Vec<Pipe>>,
    start: Position,
}

impl From<&str> for Tiles {
    fn from(s: &str) -> Self {
        let tiles: Vec<Vec<Pipe>> = s
            .lines()
            .map(|l| l.chars().map(|c| Pipe::from(c)).collect())
            .collect();
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter().enumerate().find_map(|(j, pipe)| {
                    if *pipe == Pipe::Start {
                        Some(Position(i as isize, j as isize))
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        Tiles { tiles, start }
    }
}

impl Tiles {
    fn get(&self, index: Position) -> Option<Pipe> {
        self.tiles
            .get(usize::try_from(index.0).ok()?)
            .and_then(|row| row.get(usize::try_from(index.1).ok()?))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day10::star_one(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            "4".to_string()
        );
        assert_eq!(
            Day10::star_one(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            "8".to_string()
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            Day10::star_two(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            "4".to_string()
        );
        assert_eq!(
            Day10::star_two(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            "8".to_string()
        );
        assert_eq!(
            Day10::star_two(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
            ),
            "10".to_string()
        );
    }
}

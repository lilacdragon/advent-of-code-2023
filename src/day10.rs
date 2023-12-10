use crate::DaySolution;
use std::ops::{Add, Index};

pub struct Day10;

const CROSS_OFFSETS: [(Direction, (isize, isize)); 4] = [
    (Direction::North, (-1, 0)),
    (Direction::South, (1, 0)),
    (Direction::East, (0, 1)),
    (Direction::West, (0, -1)),
];

impl DaySolution for Day10 {
    fn star_one(input: &str) -> String {
        let tiles = Tiles::from(input);

        let (mut current_position, mut last_direction) = CROSS_OFFSETS
            .iter()
            .find_map(|(direction, offset)| {
                let next_position = tiles.start + *offset;
                if let Some(pipe) = next_position.and_then(|p| tiles.get(p)) {
                    if pipe.connects_in_direction(direction.opposite()) {
                        Some((next_position.unwrap(), direction.opposite()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();
        let mut steps = 1;

        while tiles.get(current_position).unwrap() != Pipe::Start {
            for d in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                if d == last_direction {
                    continue;
                }
                let (next_position, next_direction) = CROSS_OFFSETS
                    .iter()
                    .find_map(|(direction, offset)| {
                        if *direction == d {
                            Some((current_position + *offset, *direction))
                        } else {
                            None
                        }
                    })
                    .unwrap();
                if let Some(_) = next_position.and_then(|p| tiles.get(p)) {
                    if tiles
                        .get(current_position)
                        .unwrap()
                        .connects_in_direction(next_direction)
                    {
                        current_position = next_position.unwrap();
                        last_direction = next_direction.opposite();
                        break;
                    }
                }
            }
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
    type Output = Option<Self>;

    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        let (x, y) = (self.0 + x, self.1 + y);
        if x < 0 || y < 0 {
            None
        } else {
            Some(Position(x, y))
        }
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
            .get(index.0 as usize)
            .and_then(|row| row.get(index.1 as usize))
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
        assert_eq!(Day10::star_two(""), "".to_string());
    }
}

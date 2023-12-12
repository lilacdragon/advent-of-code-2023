use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::DaySolution;

pub struct Day12;

impl DaySolution for Day12 {
    fn star_one(input: &str) -> String {
        let tiles: Vec<Row> = input.lines().map(Row::from).collect();

        tiles
            .into_par_iter()
            .map(|row| row.possible_arrangements())
            .sum::<usize>()
            .to_string()
    }
    fn star_two(input: &str) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Operational,
    Damaged,
    Unknown,
}

struct Row {
    tiles: Vec<Tile>,
    target_groups: Vec<usize>,
    unknown_idxs: Vec<usize>,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let (tiles, target_groups) = value.split_once(' ').unwrap();
        let tiles: Vec<Tile> = tiles
            .chars()
            .map(|c| match c {
                '.' => Tile::Operational,
                '#' => Tile::Damaged,
                '?' => Tile::Unknown,
                _ => unreachable!(),
            })
            .collect();
        let target_groups = target_groups
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let unknown_idxs = tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == Tile::Unknown)
            .map(|(i, _)| i)
            .collect();

        Self {
            tiles,
            target_groups,
            unknown_idxs,
        }
    }
}

impl Row {
    fn groups(tiles: &[Tile]) -> Vec<usize> {
        let mut groups = Vec::new();
        let mut current = 0;
        for tile in tiles {
            match tile {
                Tile::Damaged => current += 1,
                _ => {
                    if current > 0 {
                        groups.push(current);
                        current = 0;
                    }
                }
            }
        }
        if current > 0 {
            groups.push(current);
        }
        groups
    }
    fn possible_arrangements(&self) -> usize {
        fn recurse(
            tiles: &[Tile],
            unknown_fills: Vec<Tile>,
            unknown_idxs: &[usize],
            target_groups: &[usize],
        ) -> usize {
            if unknown_fills.len() != unknown_idxs.len() {
                let mut next_is_operational = unknown_fills.clone();
                next_is_operational.push(Tile::Operational);
                let mut next_is_damaged = unknown_fills.clone();
                next_is_damaged.push(Tile::Damaged);
                return recurse(tiles, next_is_operational, unknown_idxs, target_groups)
                    + recurse(tiles, next_is_damaged, unknown_idxs, target_groups);
            }

            let mut tiles = tiles.to_vec();
            for (idx, tile) in unknown_idxs.iter().zip(unknown_fills.iter()) {
                tiles[*idx] = *tile;
            }
            let groups = Row::groups(&tiles);
            if groups == *target_groups {
                return 1;
            }
            0
        }

        recurse(
            &self.tiles,
            Vec::new(),
            &self.unknown_idxs,
            &self.target_groups,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day12::star_one(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            "21".to_string()
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(Day12::star_two(""), "".to_string());
    }
}

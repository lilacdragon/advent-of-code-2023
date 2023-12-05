use std::ops::Range;

use crate::DaySolution;

pub struct Day5;

impl DaySolution for Day5 {
    fn part1(input: &str) -> String {
        let (seeds, maps) = input.split_once("\n\n").unwrap();

        let seeds: Vec<i64> = seeds
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let maps: Vec<_> = maps.split("\n\n").map(parse_map).collect();

        maps.into_iter()
            .fold(seeds, |cur, cur_map| {
                cur.into_iter()
                    .map(|val| {
                        cur_map
                            .iter()
                            .find(|(range, _)| range.contains(&val))
                            .map(|(_, offset)| val + offset)
                            .unwrap_or(val)
                    })
                    .collect()
            })
            .into_iter()
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(input: &str) -> String {
        todo!()
    }
}

fn parse_map(map: &str) -> Vec<(Range<i64>, i64)> {
    map.lines()
        .skip(1)
        .map(|line| {
            let [a, b, c] = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap();
            (b..b + c, a - b)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day5::part1(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "35"
        );
    }
    #[test]
    fn test_part2() {}
}

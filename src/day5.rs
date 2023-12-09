use std::ops::Range;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::DaySolution;

pub struct Day5;

impl DaySolution for Day5 {
    fn star_one(input: &str) -> String {
        let (seeds, maps) = input.split_once("\n\n").unwrap();

        let seeds: Vec<i64> = seeds
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let maps: Vec<_> = maps.split("\n\n").map(parse_map).collect();

        find_smallest_location(seeds, &maps).to_string()
    }

    fn star_two(input: &str) -> String {
        let (seeds, maps) = input.split_once("\n\n").unwrap();

        let seeds: Vec<Range<i64>> = seeds
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples()
            .map(|(start, len)| start..start + len)
            .collect();

        let maps: Vec<_> = maps.split("\n\n").map(parse_map).collect();

        let min_per_range = seeds
            .into_iter()
            .map(|range| find_smallest_location(range.collect(), &maps));

        min_per_range.min().unwrap().to_string()
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

fn find_smallest_location(seeds: Vec<i64>, maps: &Vec<Vec<(Range<i64>, i64)>>) -> i64 {
    seeds
        .into_par_iter()
        .map(|seed| {
            maps.iter().fold(seed, |cur, cur_map| {
                cur_map
                    .iter()
                    .find(|(range, _)| range.contains(&cur))
                    .map(|(_, offset)| cur + offset)
                    .unwrap_or(cur)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day5::star_one(
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
    fn test_star_two() {
        assert_eq!(
            Day5::star_two(
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
            "46"
        );
    }
}

use crate::DaySolution;

pub struct Day6;

impl DaySolution for Day6 {
    fn part1(input: &str) -> String {
        let (times, distances) = input.split_once("\n").unwrap();
        let times = times.split_whitespace().skip(1).map(|x| x.parse().unwrap());
        let distances = distances
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap());
        let races: Vec<(u32, u32)> = times.zip(distances).collect();

        races
            .into_iter()
            .map(|(t, d)| {
                let smallest_factor = (1..).find(|x| x * (t - x) > d).unwrap();
                let other_factor = t - smallest_factor;
                other_factor - smallest_factor + 1
            })
            .product::<u32>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        let (time, distance) = input.split_once("\n").unwrap();
        let time = time
            .split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let distance = distance
            .split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let smallest_factor = (1..).find(|x| x * (time - x) > distance).unwrap();
        let other_factor = time - smallest_factor;
        (other_factor - smallest_factor + 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day6::part1(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            "288".to_string()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day6::part2(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            "71503".to_string()
        );
    }
}

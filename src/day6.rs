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

        races.into_iter().map(|(t, d)| {
	    let smallest_factor = (1..).find(|x| x * (t - x) > d).unwrap();
	    let other_factor = t - smallest_factor;
	    other_factor - smallest_factor + 1
	}).product::<u32>().to_string()
    }

    fn part2(input: &str) -> String {
        todo!()
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
    fn test_part2() {}
}

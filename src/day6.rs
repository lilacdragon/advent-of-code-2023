use crate::DaySolution;

pub struct Day6;

impl DaySolution for Day6 {
    fn part1(input: &str) -> String {
        todo!()
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

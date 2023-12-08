use crate::DaySolution;

pub struct Day8;

impl DaySolution for Day8 {
    fn part1(input: &str) -> String {
        "".to_string()
    }

    fn part2(input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day8::part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            ""
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day8::part2(""), "");
    }
}

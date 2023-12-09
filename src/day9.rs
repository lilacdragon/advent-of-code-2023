use crate::DaySolution;

pub struct Day9;

impl DaySolution for Day9 {
    fn part1(input: &str) -> String {
        todo!()
    }

    fn part2(input: &str) -> String {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day9::part1(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "114"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day9::part2(""), "");
    }
}

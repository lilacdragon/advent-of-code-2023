use crate::DaySolution;

pub struct Day10;

impl DaySolution for Day10 {
    fn star_one(input: &str) -> String {
        "".to_string()
    }

    fn star_two(input: &str) -> String {
        "".to_string()
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

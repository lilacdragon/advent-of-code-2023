use crate::DaySolution;

pub struct Day1;

impl DaySolution for Day1 {
    fn part1(input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let nums: Vec<_> = l.chars().filter(|c| c.is_digit(10)).collect();
                let first = nums[0];
                let last = nums.last().unwrap();
                format!("{first}{last}").parse::<u32>().unwrap()
            })
            .sum::<u32>()
            .to_string()
    }
    fn part2(input: &str) -> String {
        let replacements = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];
        let mut fixed = String::new();
        for mut i in 0..input.len() - 1 {
            let mut replaced = false;
            for (find, replace) in &replacements {
                if input[i..].starts_with(find) {
                    fixed += replace;
                    replaced = true;
                    i += find.len();
                    break;
                } else {
                }
            }
            if !replaced {
                fixed += &input.chars().nth(i).unwrap().to_string();
            }
        }
        Self::part1(&fixed)
    }
}

#[test]
fn part_1() {
    assert_eq!(
        Day1::part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        ),
        "142"
    );
}
#[test]
fn part_2() {
    assert_eq!(
        Day1::part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        ),
        "281"
    )
}

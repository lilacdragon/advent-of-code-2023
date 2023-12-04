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
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        input
            .lines()
            .map(|l| {
                let chars: Vec<_> = l.chars().collect();
                let mut nums = Vec::new();
                for i in 0..chars.len() {
                    if chars[i].is_digit(10) {
                        nums.push(chars[i].to_digit(10).unwrap());
                        continue;
                    }
                    for (j, w) in words.iter().enumerate() {
                        if l[i..].starts_with(w) {
                            nums.push((j + 1) as u32);
                            break;
                        }
                    }
                }
                format!("{}{}", nums[0], nums.last().unwrap())
                    .parse::<u32>()
                    .unwrap()
            })
            .sum::<u32>()
            .to_string()
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

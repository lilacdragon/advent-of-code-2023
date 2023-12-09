use crate::DaySolution;

pub struct Day1;

impl DaySolution for Day1 {
    fn star_one(input: &str) -> String {
        input
            .lines()
            .map(|l| {
                let first = l
                    .chars()
                    .find(|c| c.is_digit(10))
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let last = l
                    .chars()
                    .rev()
                    .find(|c| c.is_digit(10))
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                first * 10 + last
            })
            .sum::<u32>()
            .to_string()
    }
    fn star_two(input: &str) -> String {
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        input
            .lines()
            .map(|l| {
                let chars: Vec<_> = l.chars().collect();
                let first = 'l: {
                    for i in 0.. {
                        if chars[i].is_digit(10) {
                            break 'l chars[i].to_digit(10).unwrap();
                        }
                        for (j, w) in words.iter().enumerate() {
                            if l[i..].starts_with(w) {
                                break 'l (j + 1) as u32;
                            }
                        }
                    }
                    unreachable!()
                };
                let last = 'l: {
                    for i in (0..chars.len()).rev() {
                        if chars[i].is_digit(10) {
                            break 'l chars[i].to_digit(10).unwrap();
                        }
                        for (j, w) in words.iter().enumerate() {
                            if l[..=i].ends_with(w) {
                                break 'l (j + 1) as u32;
                            }
                        }
                    }
                    unreachable!()
                };
                first * 10 + last
            })
            .sum::<u32>()
            .to_string()
    }
}

#[test]
fn part_1() {
    assert_eq!(
        Day1::star_one(
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
        Day1::star_two(
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

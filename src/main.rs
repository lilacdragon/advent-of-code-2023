mod day1;

use day1::Day1;

trait DaySolution {
    fn part1(input: &str) -> String;
    fn part2(input: &str) -> String;
}

fn main() {
    let data = include_str!("../input/day1");
    println!("{}", Day1::part1(data));
    println!("{}", Day1::part2(data));
}

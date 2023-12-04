#![allow(unused_imports)]
mod day1;
mod day2;
mod day3;

use day1::Day1;
use day2::Day2;
use day3::Day3;

trait DaySolution {
    fn part1(input: &str) -> String;
    fn part2(input: &str) -> String;
}

fn main() {
    // let data = include_str!("../input/day1");
    // println!("{}", Day1::part1(data));
    // println!("{}", Day1::part2(data));

    // let data = include_str!("../input/day2");
    // println!("{}", Day2::part1(data));
    // println!("{}", Day2::part2(data));

    let data = include_str!("../input/day3");
    println!("{}", Day3::part1(data));
    println!("{}", Day3::part2(data));
}

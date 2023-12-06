#![allow(unused_imports)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;

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

    // let data = include_str!("../input/day3");
    // println!("{}", Day3::part1(data));
    // println!("{}", Day3::part2(data));

    // let data = include_str!("../input/day4");
    // println!("{}", Day4::part1(data));
    // println!("{}", Day4::part2(data));

    let data = include_str!("../input/day5");
    println!("{}", Day5::part1(data));
    println!("{}", Day5::part2(data));
}

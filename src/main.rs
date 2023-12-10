#![allow(unused_imports)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

trait DaySolution {
    fn star_one(input: &str) -> String;
    fn star_two(input: &str) -> String;
}

fn main() {
    // let data = include_str!("../input/day1");
    // println!("{}", Day1::star_one(data));
    // println!("{}", Day1::star_two(data));

    // let data = include_str!("../input/day2");
    // println!("{}", Day2::star_one(data));
    // println!("{}", Day2::star_two(data));

    // let data = include_str!("../input/day3");
    // println!("{}", Day3::star_one(data));
    // println!("{}", Day3::star_two(data));

    // let data = include_str!("../input/day4");
    // println!("{}", Day4::star_one(data));
    // println!("{}", Day4::star_two(data));

    let data = include_str!("../input/day5");
    // println!("{}", Day5::star_one(data));
    println!("{}", Day5::star_two(data));

    // let data = include_str!("../input/day6");
    // println!("{}", Day6::star_one(data));
    // println!("{}", Day6::star_two(data));

    // let data = include_str!("../input/day7");
    // println!("{}", Day7::star_one(data));
    // println!("{}", Day7::star_two(data));

    // let data = include_str!("../input/day8");
    // println!("{}", Day8::star_one(data));
    // println!("{}", Day8::star_two(data));

    // let data = include_str!("../input/day9");
    // println!("{}", Day9::star_one(data));
    // println!("{}", Day9::star_two(data));
}

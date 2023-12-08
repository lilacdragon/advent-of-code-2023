use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use itertools::Itertools;
use regex::Regex;

use crate::DaySolution;

pub struct Day8;

impl DaySolution for Day8 {
    fn part1(input: &str) -> String {
        let (instructions, nodes) = input.split_once("\n\n").unwrap();
        let instructions = parse_instructions(instructions);
        let (nodes, start_idx, end_idx) = parse_nodes(nodes);

        let mut current_index = start_idx;
        for (steps, instruction) in instructions.into_iter().cycle().enumerate() {
            if current_index == end_idx {
                return steps.to_string();
            }
            current_index = nodes[current_index][instruction];
        }

	unreachable!()
    }

    fn part2(input: &str) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl Index<Instruction> for (usize, usize) {
    type Output = usize;

    fn index(&self, index: Instruction) -> &Self::Output {
        match index {
            Instruction::Left => &self.0,
            Instruction::Right => &self.1,
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect()
}

fn parse_nodes(input: &str) -> (Vec<(usize, usize)>, usize, usize) {
    let re = Regex::new(r"([A-Z]{3})").unwrap();

    let mut index_map = HashMap::new();
    let mut nodes = Vec::new();
    for (idx, (node, left, right)) in re.find_iter(input).map(|m| m.as_str()).tuples().enumerate() {
        index_map.insert(node.to_string(), idx);
        nodes.push((left, right));
    }
    let nodes: Vec<(usize, usize)> = nodes
        .into_iter()
        .map(|(l, r)| (index_map[l], index_map[r]))
        .collect();

    (nodes, index_map["AAA"], index_map["ZZZ"])
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
            "2"
        );
        assert_eq!(
            Day8::part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            "6"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day8::part2(""), "");
    }
}

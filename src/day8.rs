use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;

use crate::DaySolution;

pub struct Day8;

impl DaySolution for Day8 {
    fn star_one(input: &str) -> String {
        let (instructions, nodes) = input.split_once("\n\n").unwrap();
        let instructions = parse_instructions(instructions);
        let (nodes, index_map) = parse_nodes(nodes);
        let (start_idx, end_idx) = (index_map["AAA"], index_map["ZZZ"]);

        let mut current_index = start_idx;
        for (steps, instruction) in instructions.into_iter().cycle().enumerate() {
            if current_index == end_idx {
                return steps.to_string();
            }
            current_index = nodes[current_index][instruction];
        }

        unreachable!()
    }

    fn star_two(input: &str) -> String {
        let (instructions, nodes) = input.split_once("\n\n").unwrap();
        let instructions = parse_instructions(instructions);
        let (nodes, index_map) = parse_nodes(nodes);

        let starting_idxs: Vec<usize> = index_map
            .iter()
            .filter(|(name, _)| name.ends_with("A"))
            .map(|(_, idx)| *idx)
            .collect();
        let all_ending_idxs: HashSet<usize> = index_map
            .iter()
            .filter(|(name, _)| name.ends_with("Z"))
            .map(|(_, idx)| *idx)
            .collect();

        let final_idxs: Vec<usize> = starting_idxs
            .into_iter()
            .map(|mut idx| {
                for (steps, instruction) in instructions.iter().cycle().enumerate() {
                    if all_ending_idxs.contains(&idx) {
                        return steps;
                    }

                    idx = nodes[idx][*instruction];
                }
                unreachable!()
            })
            .collect();

        final_idxs.into_iter().reduce(lcm).unwrap().to_string()
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

fn parse_nodes(input: &str) -> (Vec<(usize, usize)>, HashMap<String, usize>) {
    let re = Regex::new(r"([A-Z0-9]{3})").unwrap();

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

    (nodes, index_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(
            Day8::star_one(
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
            Day8::star_one(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            "6"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            Day8::star_two(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            "6"
        );
    }
}

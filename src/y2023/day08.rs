use std::{fmt::Display, collections::HashMap};
use num::Integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::super::day::Day;

pub struct Day08
{
    instructions: Vec<bool>,
    nodes: HashMap<String, Node>,
}

impl Day08 {
    pub fn new() -> Day08
    {
        //let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let input = include_str!("../../input/y2023/08");

        let (instructions, rest) = input.split_once("\n\n").unwrap();

        let instructions = instructions.chars().map(|c| c == 'R').collect();
        let nodes = rest.lines().map(Node::from_str).map(|n| (n.id.to_string(), n)).collect();

        Day08 { instructions, nodes }
    }
}

impl Day for Day08 {
    fn day_name(&self) -> String { String::from("08") }
    fn answer1(&self) -> String { String::from("23147") }
    fn answer2(&self) -> String { String::from("22289513667691") }

    fn part1(&mut self) -> String {
        self.find_target("AAA", "ZZZ").to_string()
    }

    fn part2(&mut self) -> String {
        self.nodes.par_iter()
            .filter(|(n, _)| n.ends_with('A'))
            .map(|(n, _)| self.find_target(n, "Z"))
            .reduce(|| 1, |acc, n| acc.lcm(&n))
            .to_string()
    }
}

impl Day08 {
    fn find_target(&self, start: &str, ends_with: &str) -> usize {
        let mut at = start.to_string();
        let mut steps = 0;
        while !at.ends_with(ends_with) {
            let instruction = self.instructions[steps % self.instructions.len()];
            let node = self.nodes.get(&at).unwrap();
            at = node.next_node(instruction);
            steps += 1;
        }
        steps
    }
}


#[derive(Hash)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn from_str(input: &str) -> Node {
        let (id, rest) = input[..(input.len()-1)].split_once(" = (").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        Node { id: id.to_string(), left: left.to_string(), right: right.to_string() }
    }

    fn next_node(&self, go_right: bool) -> String {
        if go_right {
            self.right.to_string()
        } else {
            self.left.to_string()
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}, {}", self.id, self.left, self.right)
    }
}
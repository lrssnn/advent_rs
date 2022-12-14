use std::fmt::Display;
use std::collections::{HashMap, HashSet};

use super::super::day::Day;

pub struct Day13
{
    packets: Vec<Packet>,
}

impl Day13 {
    pub fn new() -> Day13
    {
        //let input = include_str!("input13");
        let input = include_str!("input13_example");

        let packets = input.trim().split('\n')
            .map(Packet::from_str)
            .collect::<Vec<_>>();

        Day13 { packets }
    }
}

impl Day for Day13 {
    fn day_name(&self) -> String { String::from("13") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        let ans1 = 0;
        let ans2 = 0;

        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day13 {
}

struct Packet {
    contents: Vec<PacketData>,
}

enum PacketData {
    Number(usize),
    List(Vec<PacketData>),
}

impl Packet {
    fn from_str(input: &str) -> Packet {
        // outer element is always a list, strip off []
        let mut contents = Vec::new();
        println!("{input}"); 
        
        Packet { contents }
    }
}

/*
impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
*/
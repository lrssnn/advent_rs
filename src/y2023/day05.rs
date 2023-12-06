use std::fmt::Display;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};

use super::super::day::Day;

pub struct Day05
{
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Day05 {
    pub fn new() -> Day05
    {
        let input = include_str!("../../input/y2023/05_example");
        //let input = include_str!("../../input/y2023/05");

        let (first, rest) = input.split_once("\n\n").unwrap();
        let seeds = first[7..].split(' ').map(|n| n.parse().unwrap()).collect();
        let maps = rest.split("\n\n").map(|m| Map::from_str(m)).collect();

        Day05 { seeds, maps }
    }
}

impl Day for Day05 {
    fn day_name(&self) -> String { String::from("05") }
    fn answer1(&self) -> String { String::from("324724204") }
    fn answer2(&self) -> String { String::from("104070862") }

    fn part1(&mut self) -> String {
        self.seeds.iter().map(|s|
            chained_map(*s, &self.maps)
        ).min().unwrap().to_string()
    }

    fn part2(&mut self) -> String {
        let mut seed_ranges = vec![];
        for i in 0..self.seeds.len() {
            if i % 2 == 0 {
                seed_ranges.push(self.seeds[i]..=(self.seeds[i] + self.seeds[i + 1]));
            }
        }

        let answer = seed_ranges.par_iter()
        .map(|r| {
            r.clone().into_par_iter()
            .map(|s| {
                chained_map(s, &self.maps)
            }
            ).min().unwrap()
        }
        ).min().unwrap();

        if answer >= 104070863 { println!("TOO BIG"); }
        answer.to_string()
    }
}

fn chained_map(seed: usize, maps: &[Map]) -> usize {
    maps.iter().fold(seed, |value, map| map.map(value))
}

struct Map {
    src: String,
    dest: String, 
    ranges: Vec<MapRange>,
}

struct MapRange {
    src: usize,
    dest: usize,
    len: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let (first, rest) = input.split_once('\n').unwrap();
        let (src, dest) = first[..(first.len() - 5)].split_once("-to-").unwrap();
        let ranges = rest.lines().map(MapRange::from_str).collect();
        Self { src: src.to_string(), dest: dest.to_string(), ranges }
    }

    fn map(&self, input: usize) -> usize {
        self.ranges.iter().find_map(|r| r.try_map(input)).unwrap_or(input)
    }
}

impl MapRange {
    fn from_str(input: &str) -> Self {
        let parts = input.split(' ').collect::<Vec<_>>();
        Self { 
            src: parts[1].parse::<usize>().unwrap(), 
            dest: parts[0].parse::<usize>().unwrap(), 
            len: parts[2].parse::<usize>().unwrap(), 
        }
    }

    fn try_map(&self, input: usize) -> Option<usize> {
        //println!("Trying to map {input} in {}-{}", self.src, self.src + self.len);
        if input >= self.src && input < self.src + self.len {
            let input_offset = input - self.src;
            Some(self.dest + input_offset)
        } else {
            None
        }
    }
}

impl Display for MapRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.dest, self.src, self.len)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-to-{}\n{}", self.src, self.dest,
            self.ranges.iter().map(|r| r.to_string()).reduce(|acc, e| acc + "\n" + &e).unwrap())
    }
}
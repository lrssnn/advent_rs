use std::fmt::{Debug, Display};

use super::super::day::Day;

pub struct Day22
{
    bricks: Vec<Brick>,
}

impl Day22 {
    pub fn new() -> Day22
    {
        //let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";
        let input = include_str!("../../input/y2023/22");

        let bricks = input.lines().enumerate().map(|(i, l)| Brick::from_str(l, i)).collect();

        Day22 { bricks }
    }
}

impl Day for Day22 {
    fn day_name(&self) -> String { String::from("22") }
    fn answer1(&self) -> String { String::from("437") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        // for b in &self.bricks {
        //     println!("{b}");
        // }

        // println!();
        // self._print_across_x();
        // Doesn't do anything? Which Surprises me...
        //self.bricks.sort_by_key(|b| b.cubes.iter().map(|c| c.z).min());
        let mut drops = 0;
        while self.drop_once() {
            drops += 1;
        }
        println!("Dropped {drops} times...");
        // println!();
        // self._print_across_x();
        // println!();
        // self._print_across_y();
        let answer = self.count_redundant();
        if answer >= 540 { println!("TOO HIGH") }

        // for brick in &self.bricks {
        //     println!("{} supports: ", brick.label);
        //     for b in brick.supports(&self.bricks) {
        //         println!("-> {b}");
        //         for supporter in &b.supported_by(&self.bricks) {
        //             println!("  -> {supporter}")
        //         }
        //     }

        //     println!("Redundant: {}", brick.is_redundant(&self.bricks));
            
        //     println!();
        // }
        answer.to_string()
    }

    fn part2(&mut self) -> String {
        String::new()
    }
}

impl Day22 {
    fn _print_across_x(&self) {
        let max_z = self.bricks.iter().map(|b| b.cubes.iter().map(|c| c.z).max().unwrap()).max().unwrap();
        let max_x = self.bricks.iter().map(|b| b.cubes.iter().map(|c| c.x).max().unwrap()).max().unwrap();
        let min_x = self.bricks.iter().map(|b| b.cubes.iter().map(|c| c.x).min().unwrap()).min().unwrap();
        for z in (0..=max_z).rev() {
            for x in min_x..=max_x {
                if let Some(brick) = self.bricks.iter().find(|b| b.cubes.iter().any(|c| c.x == x && c.z == z)) {
                    print!("{}", brick.label);
                } else {
                    print!(".");
                }
            }
            println!(" {z}");
        }
    }

    fn _print_across_y(&self) {
        let max_z = self.bricks.iter().map(|b| b.cubes.iter().map(|c| c.z).max().unwrap()).max().unwrap();
        let max_y = self.bricks.iter().map(|b| b.cubes.iter().map(|c| c.y).max().unwrap()).max().unwrap();
        let min_y = self.bricks.iter().map(|b| b.cubes.iter().map(|c| c.y).min().unwrap()).min().unwrap();
        for z in (0..=max_z).rev() {
            for y in min_y..=max_y {
                if let Some(brick) = self.bricks.iter().find(|b| b.cubes.iter().any(|c| c.y == y && c.z == z)) {
                    print!("{}", brick.label);
                } else {
                    print!(".");
                }
            }
            println!(" {z}");
        }
    }

    fn drop_once(&mut self) -> bool {
        let mut any_moved = false;
        // We are operating on the assumption that our list goes from low to high block
        let others = self.bricks.clone();
        for brick in &mut self.bricks {
            if brick.try_move_down(&others) {
                any_moved = true;
            }
        }
        any_moved
    }

    fn count_redundant(&self) -> usize {
        self.bricks.iter().filter(|b| b.is_redundant(&self.bricks)).count()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, PartialEq, Eq)]
struct Brick {
    cubes: Vec<Coord>,
    label: String,
}

impl Brick {
    fn from_str(input: &str, id: usize) -> Brick {
        let (left, right) = input.split_once('~').unwrap();
        let from = Coord::from_str(left);
        let to = Coord::from_str(right);
        let cubes = Brick::cubes(from, to);
        let label = Self::generate_label(id);
        Brick { cubes, label }
    }

    fn generate_label(mut id: usize) -> String {
        let mut result = String::new();
        while id >= 26 {
            result = (('A' as u8 + ((id % 26) as u8)) as char).to_string() + &result;
            id /= 26;
            id -= 1;
        }
        result = (('A' as u8 + ((id % 26) as u8)) as char).to_string() + &result;
        result
    }

    fn cubes(from: Coord, to: Coord) -> Vec<Coord> {
        // there may be a smarter way to do this
        if from.x != to.x {
            (from.x..=to.x).map(|x| Coord {x, y: from.y, z: from.z }).collect()
        } else if from.y != to.y {
            (from.y..=to.y).map(|y| Coord {x: from.x, y, z: from.z }).collect()
        } else if from.z != to.z {
            (from.z..=to.z).map(|z| Coord {x: from.x, y: from.y, z}).collect()
        } else {
            // Single cube
            vec![from]
        }
    }

    fn try_move_down(&mut self, others: &Vec<Brick>) -> bool {
        for cube in &self.cubes {
            let target_cube = Coord { x: cube.x, y: cube.y, z: cube.z - 1};
            if target_cube.z == 0 {
                return false;
            }

            if others.iter().any(|b| b != self && b.cubes.contains(&target_cube)) {
                return false;
            }
        }

        for cube in &mut self.cubes {
            cube.z -= 1;
        }
        true
    }

    fn supports(&self, bricks: &[Brick]) -> Vec<Brick> {
        bricks.iter()
            .filter(|&b| b != self)
            .filter(|b| b.cubes.iter().any(|c| self.cubes.iter().any(|cube| cube.x == c.x && cube.y == c.y && cube.z == c.z - 1)))
            .cloned()
            .collect()
    }

    fn supported_by(&self, bricks: &[Brick]) -> Vec<Brick> {
        bricks.iter()
            .filter(|&b| b != self)
            .filter(|b| b.cubes.iter().any(|c| self.cubes.iter().any(|cube| cube.x == c.x && cube.y == c.y && cube.z == c.z + 1)))
            .cloned()
            .collect()
    }

    fn is_redundant(&self, bricks: &[Brick]) -> bool {
        let supports = self.supports(bricks);

        supports.is_empty() || supports.iter().all(|b| b.supported_by(bricks).len() > 1)
    }
}

impl Coord {
    fn from_str(input: &str) -> Coord {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Coord { x, y, z }
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}  <- {}", self.cubes[0], self.cubes[self.cubes.len() - 1], self.label)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}  <- {}", self.cubes[0], self.cubes[self.cubes.len() - 1], self.label)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
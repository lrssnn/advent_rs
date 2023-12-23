use std::fmt::{Debug, Display};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelRefMutIterator};

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
    fn answer2(&self) -> String { String::from("42561") }

    fn part1(&mut self) -> String {
        // TODO this takes ages...
        while self.drop_once() {}

        self.count_redundant().to_string()
    }

    fn part2(&mut self) -> String {
        // for brick in &self.bricks {
        //     println!("{brick} => {}", brick.count_chain_reaction(&self.bricks, &vec![brick.clone()]));
        // }
        self.bricks.iter().map(|b| b.count_chain_reaction(&self.bricks)).sum::<usize>().to_string()
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

    fn count_redundant(&mut self) -> usize {
        let context = self.bricks.clone();
        let mut count = 0;
        for brick in &mut self.bricks {
            if brick.is_redundant(&context) {
                count += 1;
            } 
        }
        count
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
            result = ((b'A' + ((id % 26) as u8)) as char).to_string() + &result;
            id /= 26;
            id -= 1;
        }
        result = ((b'A' + ((id % 26) as u8)) as char).to_string() + &result;
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

    fn try_move_down(&mut self, others: &[Brick]) -> bool {
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

    fn is_redundant(&mut self, bricks: &[Brick]) -> bool {
        let supports = self.supports(bricks);

        supports.is_empty() || supports.iter().all(|b| b.supported_by(bricks).len() > 1)
    }

    fn count_chain_reaction(&self, bricks: &[Brick]) -> usize {
        //let supported_only_by_me = self.supports(bricks).into_iter().filter(|b| b.supported_by(bricks).len() == 1);

        //println!("Count chain reaction on {}", self.label);

        let mut last_dissolving = vec![self.clone()];

        let mut new_dissolves = bricks.iter()
            .filter(|b| !last_dissolving.contains(b))
            .filter(|b| !b.supported_by(bricks).is_empty())
            .filter(|b| 
                b.supported_by(bricks).iter()
                .all(|supporter| last_dissolving.contains(supporter))
            )
            .map(|b| b.clone())
            .collect::<Vec<Brick>>();

        while !new_dissolves.is_empty() {
            //println!("found {} new dissolves: ", new_dissolves.len());
            //for b in &new_dissolves { println!("  {}", b.label); }
            last_dissolving.extend(new_dissolves.into_iter());
            //println!("New dissolving set:");
            //for b in &last_dissolving { println!("  {}", b.label); }

            new_dissolves = bricks.iter()
                .filter(|b| !last_dissolving.contains(b))
                .filter(|b| !b.supported_by(bricks).is_empty())
                .filter(|b| 
                    b.supported_by(bricks).iter()
                    .all(|supporter| last_dissolving.contains(supporter))
                )
                .cloned()
                .collect::<Vec<Brick>>();
        }

        last_dissolving.len() - 1 // Uncount the original dissolving brick
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
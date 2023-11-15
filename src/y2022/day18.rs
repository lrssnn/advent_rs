use std::{fmt::Display, collections::VecDeque};

use super::super::day::Day;

pub struct Day18
{
    cubes: Vec<Cube>,
    air_cubes: Option<Vec<Cube>>,
}

impl Day18 {
    pub fn new() -> Day18
    {
        let input = include_str!("input18");
        //let input = "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5";

        let cubes = input.lines()
            .map(Cube::from_str).collect::<Vec<_>>();

        Day18 { cubes, air_cubes: None }
    }
}

impl Day for Day18 {
    fn day_name(&self) -> String { String::from("18") }
    fn answer1(&self) -> String { String::from("3466") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        let air_cubes: Vec<Cube> = self.cubes.iter()
            .flat_map(|cube| cube.neighbours().iter()
                .filter(|n| !self.cubes.contains(n)).copied().collect::<Vec<_>>()
            ).collect();

        let ans = air_cubes.len().to_string();
        self.air_cubes = Some(air_cubes);

        ans
    }

    fn part2(&mut self) -> String {
        let air_cubes = self.air_cubes.as_ref().unwrap();

        let mut contained_air: Vec<Cube> = air_cubes.iter()
            .filter(|cube| cube.neighbours().iter()
                .all(|n| self.cubes.contains(n) || air_cubes.contains(n))
            ).copied().collect();

        contained_air.dedup();
        

        //println!("{} contained cubes", contained_air.len());
        

        // we need to remove any interior cubes. To do that, grab all the air cubes that can
        // see at least one cube that is not contained in the air_cubes or the cubes themselves
        // This should mean they can see the outer air. If we have (for example) a 3x3 contained inside the shape, then it won't work
        /*
        let exterior_cubes: Vec<Cube> = air_cubes.iter()
            .filter(|cube| cube.neighbours().iter()
                .any(|n| !self.cubes.contains(n) && !air_cubes.contains(n))
            ).copied().collect(); 
            */

        // Find a cube that's definitley on the outside of the shape. This is sufficient
        /*
        let min_x = air_cubes.iter().min_by_key(|c| c.x).unwrap();
        let reachable = Self::reachable(&mut self.cubes, min_x);
        */


        // I think there could be air pockets of more than size one that will still be in here.
        // filter them out by looking for neighbours that are in the list

        /*
        let exterior_surface_area: usize = exterior_cubes.iter()
            .filter(|cube| cube.neighbours().iter()
                .all(|n| !exterior_cubes.contains(n)))
                */

        (air_cubes.len() - contained_air.len()).to_string()
    }
}

impl Day18 {
    #![allow(dead_code)]
    fn reachable(cubes: &[Cube], seed: &Cube) -> Vec<Cube> {
        let mut result = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(*seed);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            result.push(current);
            for neighbour in current.neighbours() {
                if cubes.contains(&neighbour) && !queue.contains(&neighbour) && !result.contains(&neighbour) {
                    queue.push_back(neighbour);
                }
            }
        }
        
        result
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn from_str(input: &str) -> Cube {
        let mut parts = input.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        let z = parts.next().unwrap().parse::<i32>().unwrap();
        Cube {x, y, z}
    }

    fn neighbours(&self) -> [Cube; 6] {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        [
            Cube { x: x - 1, y, z}, Cube { x: x + 1, y, z},
            Cube { x, y: y - 1, z}, Cube { x, y: y + 1, z},
            Cube { x, y, z: z - 1}, Cube { x, y, z: z + 1},
        ]
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
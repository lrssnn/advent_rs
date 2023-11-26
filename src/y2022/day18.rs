use std::{fmt::Display, collections::HashSet};

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

    fn lies_within(neighbour: Cube, min_x: i32, min_y: i32, min_z: i32, max_x: i32, max_y: i32, max_z: i32) -> bool {
        neighbour.x >= min_x && neighbour.x <= max_x &&
        neighbour.y >= min_y && neighbour.y <= max_y &&
        neighbour.z >= min_z && neighbour.z <= max_z
    }

}

impl Day for Day18 {
    fn day_name(&self) -> String { String::from("18") }
    fn answer1(&self) -> String { String::from("3466") }
    fn answer2(&self) -> String { String::from("2012") }

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
        // Strategy: Flood fill from a known outside point, counting the amount of times we touch the structure
        let known_air = self.air_cubes.as_ref().unwrap().iter().min_by_key(|c| c.x).unwrap();
        let mut to_process = Vec::new();
        let mut seen = HashSet::new();

        to_process.push(*known_air);

        let mut surface_area = 0;

        // Anything outside these bounds cannot possibly contribute
        let min_x = self.cubes.iter().min_by_key(|c| c.x).unwrap().x - 1;
        let min_y = self.cubes.iter().min_by_key(|c| c.y).unwrap().y - 1;
        let min_z = self.cubes.iter().min_by_key(|c| c.z).unwrap().z - 1;
        let max_x = self.cubes.iter().max_by_key(|c| c.x).unwrap().x + 1;
        let max_y = self.cubes.iter().max_by_key(|c| c.y).unwrap().y + 1;
        let max_z = self.cubes.iter().max_by_key(|c| c.z).unwrap().z + 1;

        while let Some(processing) = to_process.pop() {
            seen.insert(processing);
            for neighbour in processing.neighbours() {
                // If this neighbour touches the droplet, count the cooling power
                if self.cubes.contains(&neighbour) {
                    surface_area += 1;
                    continue;
                }
                // We know its air, add it to the processing queue IF there is any point
                if !seen.contains(&neighbour) // Don't process one square more than once
                    && !to_process.contains(&neighbour) // Don't queue one square more than once
                    && Self::lies_within(neighbour, min_x, min_y, min_z, max_x, max_y, max_z) { // Don't walk away from the droplet
                    to_process.push(neighbour);
                }
            }
        }

        surface_area.to_string()
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
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
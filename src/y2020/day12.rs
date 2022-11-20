use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day12 {
    steps: Vec<Step>,
    x: isize,
    y: isize,
    facing: Direction,
    way_x: isize,
    way_y: isize,
}

impl Day12 {
    pub fn new() -> Day12 {
        let input = fs::read_to_string("src/y2020/input12").expect("File Read Error");
    
        //let input = "F10\nN3\nF7\nR90\nF11";

        let lines = input.trim().split('\n').map(|s| s.trim());

        let steps = lines.map(Step::from_string).collect();
        
        Day12 { steps, x: 0, y: 0, facing: Direction::East, way_x: 10, way_y: 1 }
    }

    fn get_distance(&mut self) -> isize {
        for step in &self.steps {
            (self.x, self.y, self.facing) = self.process(step);
        }

        self.x.abs() + self.y.abs()
    }

    fn get_distance_way(&mut self) -> isize {
        for step in &self.steps {
            (self.x, self.y, self.way_x, self.way_y) = self.process_way(step);
        }

        self.x.abs() + self.y.abs()
    }

    fn process(&self, step: &Step) -> (isize, isize, Direction) {
        let (mut x, mut y, mut facing) = (self.x, self.y, self.facing);
        match step.direction {
            Direction::North => { y += step.distance },
            Direction::South => { y -= step.distance },
            Direction::East => { x += step.distance },
            Direction::West => { x -= step.distance },
            Direction::Left => { facing = facing.left_turns(step.distance) },
            Direction::Right => { facing = facing.right_turns(step.distance) },
            Direction::Forward => { match facing {
                Direction::North => { y += step.distance },
                Direction::South => { y -= step.distance },
                Direction::East => { x += step.distance },
                Direction::West => { x -= step.distance },
                _ => panic!("Invalid Facing"),
            } },
        }
        //println!("Processed '{}'. Location ({},{}) Facing {}", step, x, y, facing);
        (x, y, facing)
    }

    fn process_way(&self, step: &Step) -> (isize, isize, isize, isize) {
        let (mut x, mut y, mut way_x, mut way_y) = (self.x, self.y, self.way_x, self.way_y);
        match step.direction {
            Direction::North => { way_y += step.distance },
            Direction::South => { way_y -= step.distance },
            Direction::East => { way_x += step.distance },
            Direction::West => { way_x -= step.distance },
            Direction::Left => { (way_x, way_y) = Self::rotate_way_left(way_x, way_y, step.distance) },
            Direction::Right => { (way_x, way_y) = Self::rotate_way_right(way_x, way_y, step.distance) },
            Direction::Forward => { 
                x += step.distance * way_x; 
                y += step.distance * way_y
            } 
        }
        //println!("Processed '{}'. Location ({},{}) Way ({}, {})", step, x, y, way_x, way_y);
        (x, y, way_x, way_y)
    }

    fn rotate_way_left(mut x: isize, mut y: isize, degs: isize) -> (isize, isize) {
        let turns = degs / 90;
        for _ in 0..turns {
            let oldy = y;
            y = x;
            x = -oldy;
        }
        (x, y)
    }

    fn rotate_way_right(mut x: isize, mut y: isize, degs: isize) -> (isize, isize) {
        let turns = degs / 90;
        for _ in 0..turns {
            let oldx = x;
            x = y;
            y = -oldx;
        }
        (x, y)
    }
}

impl Day for Day12 {
    fn day_name(&self) -> String { String::from("12") }
    fn answer1(&self) -> String { String::from("938") }
    fn answer2(&self) -> String { String::from("54404") }

    fn solve(&mut self) -> (String, String) {
        //for step in &self.steps { println!("{}", step);}
        let part1 = self.get_distance();
        self.x = 0; self.y = 0;
        let part2 = self.get_distance_way();

        //println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

struct Step {
    direction: Direction,
    distance: isize,
}

#[derive(Clone, Copy)]
enum Direction {
    North, South, East,West,
    Left, Right, Forward,
}

impl Step {
    fn from_string(input: &str) -> Step {
        let mut chars = input.chars();
        let direction = match chars.next().expect("???") {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => panic!("!!!"),
        };

        let distance = chars.as_str().parse::<isize>().expect("!!!");

        Step { distance, direction }
    }
}

impl Direction {
    fn left_turns(&self, degs: isize) -> Direction {
        // Turns are always in multiples of 90 degrees, thank god
        let turns = degs / 90;
        let mut d: Direction = *self;
        for _ in 0..turns {
            d = Direction::left(d);
        }

        //println!("Turning {} {} deg left gives {}", self, degs, d);

        d
    }

    fn left(d: Direction) -> Direction {
        match d {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            _ => panic!("Invalid Facing"),
        }
    }

    fn right_turns(&self, degs: isize) -> Direction {
        // Turns are always in multiples of 90 degrees, thank god
        let turns = degs / 90;
        let mut d: Direction = *self;
        for _ in 0..turns {
            d = Direction::right(d);
        }

        //println!("Turning {} {} deg right gives {}", self, degs, d);

        d
    }

    fn right(d: Direction) -> Direction {
        match d {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            _ => panic!("Invalid Facing"),
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
            Direction::Left => 'L',
            Direction::Right => 'R',
            Direction::Forward => 'F',
        })
    }
}

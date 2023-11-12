use std::fmt::Display;
use crate::search::*;

use super::super::day::Day;

/*
const WIDTH: usize = 8;
const HEIGHT: usize = 5;
*/
const WIDTH: usize = 101;
const HEIGHT: usize = 41;

pub struct Day12
{
    heightmap: [[u8; WIDTH]; HEIGHT],
}

impl Day12 {
    pub fn new() -> Day12
    {
        let input = include_str!("input12");
        //let input = include_str!("input12_example");

        let heightmap = input.trim().split('\n')
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>().try_into().unwrap())
            .collect::<Vec<_>>().try_into().unwrap();

        Day12 { heightmap }
    }
}

impl Day for Day12 {
    fn day_name(&self) -> String { String::from("12") }
    fn answer1(&self) -> String { String::from("361") }
    fn answer2(&self) -> String { String::from("354") }

    fn part1(&mut self) -> String {
        let (start, target) = self.find_endpoints();

        let path = self.find_shortest_path(&vec![start], target);

        (path.len() - 1).to_string() // minus one because we count nodes and answer is in STEPS (not including starting node)
    }

    fn part2(&mut self) -> String {
        // OPTIMISATION: redoing work
        let (_, target) = self.find_endpoints();
        let all_starts = self.find_startpoints();
        let path_two = self.find_shortest_path(&all_starts, target);
        (path_two.len() - 1).to_string()
    }
}

impl Day12 {
    fn find_endpoints(&self) -> (Point, Point){
        let mut start = Point(0, 0);
        let mut target = Point(0, 0);

        for y in 0..self.heightmap.len() { 
            for x in 0..self.heightmap[y].len() {
                if self.heightmap[y][x] == b'S' {
                    start = Point(x, y);
                } else if self.heightmap[y][x] == b'E' {
                    target = Point(x, y);
                }
            }
        }
        (start, target)
    }
    
    fn find_startpoints(&self) -> Vec<Point> {
        let mut result = vec![];

        for y in 0..self.heightmap.len() { 
            for x in 0..self.heightmap[y].len() {
                if self.heightmap[y][x] == b'a' {
                    result.push(Point(x ,y));
                }
            }
        }
        result
    }
    
    fn find_shortest_path(&self, start: &Vec<Point>, target: Point) -> Vec<Point> {
        let mut best_path = vec![];
        for &point in start {
            let give_up_threshold = if best_path.is_empty() { usize::MAX } else { best_path.len()};
            if let Some(path) = astar_search(&point, 
                |_p| 1, // No heuristic
                |&p| self.neighbours(p),
                |&p| p == target,
                give_up_threshold)
                {
                if best_path.is_empty() || path.len() < best_path.len() {
                    best_path = path;
                }
            }
        }
        
        best_path
    }

    fn neighbours(&self, p: Point) -> Vec<Point> {
        let mut result = vec![];
        let max_height = self.get_height(p) + 1; // We can step one higher than our current height
        
        for neighbour in p.neighbours() {
            if self.get_height(neighbour) <= max_height {
                result.push(neighbour)
            }
        }
        
        result
    }

    fn get_height(&self, point: Point) -> u8 {
        let mut height = self.heightmap[point.1][point.0];

        // Correct for endpoints. This is horrible lol
        if height == b'S' { height = b'a' };
        if height == b'E' { height = b'z' };

        height
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point(usize, usize);

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let mut result = vec![];
        // Down (y + 1)
        if self.1 != HEIGHT - 1 { result.push(self.down()); }
        // Up (y - 1)
        if self.1 != 0 { result.push(self.up()); }
        // Left (x - 1)
        if self.0 != 0 { result.push(self.left()); }
        // Right (x + 1)
        if self.0 != WIDTH - 1 { result.push(self.right()); }
        
        result
    }
    // Can panic, maybe nicer to move checks into here?
    fn down(&self) -> Point { Point(self.0, self.1 + 1) }
    fn up(&self) -> Point { Point(self.0, self.1 - 1) }
    fn left(&self) -> Point { Point(self.0 - 1, self.1) }
    fn right(&self) -> Point { Point(self.0 + 1, self.1) }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
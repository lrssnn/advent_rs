use std::fmt::Display;
use std::collections::{HashMap, BinaryHeap, HashSet};

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

    fn solve(&mut self) -> (String, String)
    {
        let (start, target) = self.find_endpoints();

        let path = self.find_shortest_path(&vec![start], target);

        let ans1 = path.len() - 1; // minus one because we count nodes and answer is in STEPS (not including starting node)

        let all_starts = self.find_startpoints();
        let path_two = self.find_shortest_path(&all_starts, target);
        let ans2 = path_two.len() - 1;

        //println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
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
            if let Some(path) = self.a_star(point, target, if best_path.is_empty() { usize::MAX } else { best_path.len()}) {
                if best_path.is_empty() || path.len() < best_path.len() {
                    best_path = path;
                }
            }
        }
        
        best_path
    }


    fn a_star(&self, start: Point, target: Point, max_len: usize) -> Option<Vec<Point>> {
        // Copied from pseudoCode on wikipedia, plus slight rust help from `pathfinding` crate source.
        // But that is more optimised. Complate lack of borrows here in favour of copies feels so not rust

        // The set of discovered points that need to be expanded. Intially, only start is known.
        // If this becomes a BinaryHeap, it becomes faster to find 'current' below
        let mut open_set = HashSet::new();
        open_set.insert(start);

        // For Point p, came_from[p] is the point immediately preceding it on the cheapest path from
        // start to p currently known
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        // for point p, g_score[p] is the cost of the cheapest path from start to n currently known
        let mut g_score: HashMap<Point, usize> = HashMap::new();
        g_score.insert(start, 0);

        // f_score would be g_score modified by a heuristic, but we don't have one, so just ignore it

        while !open_set.is_empty() {
            let current: Point = *open_set.iter()
                .min_by_key(|&point| g_score.get(point).unwrap_or(&usize::MAX))
                .unwrap();

            if current == target {
                return Some(Self::reconstruct_path(&came_from, current));
            }

            open_set.remove(&current);
            for neighbour in self.neighbours(current) {
                // We would calculate weight here, but we don't have any
                let tentative_g_score = g_score.get(&current).unwrap() + 1;
                // Give up early if we have gone longer than our currently known shortest
                let give_up = tentative_g_score >= max_len;
                
                if !give_up && tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    // Update f_score here if we include it
                    
                    open_set.insert(neighbour);
                }
            }
        }
        None
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

    fn reconstruct_path(came_from: &HashMap<Point, Point>, endpoint: Point) -> Vec<Point> {
        // Following the path set out in came_from
        let mut result: Vec<Point> = vec![];
        let mut current = endpoint;
        loop {
            result.push(current);
            match came_from.get(&current) {
                Some(&parent) => {
                    current = parent;
                },
                None => {
                    result.reverse();
                    return result; 
                }
            }
        }
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
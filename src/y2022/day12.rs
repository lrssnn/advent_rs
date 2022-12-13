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
        //println!("{},{}", input.split("\n").count(), input.split("\n").next().expect("!").len());
        //let input = include_str!("input12_example");

        let heightmap = input.trim().split("\n")
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>().try_into().unwrap())
            .collect::<Vec<_>>().try_into().unwrap();

        Day12 { heightmap }
    }
}

impl Day for Day12 {
    fn day_name(&self) -> String { String::from("12") }
    fn answer1(&self) -> String { String::from("361") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        // for line in self.heightmap { 
        //     for c in line {
        //         print!("{}", c as char);
        //     }
        //     println!();
        // }


        let (start, target) = self.find_endpoints();
        //println!("{start} -> {target}");

        let path = self.a_star(start, target).unwrap();

        // for point in &path {
        //     println!("{point}");
        // }

        let ans1 = path.len() - 1; // minus one because we count nodes and answer is in STEPS (not including starting node)
        let ans2 = 0;

        //println!("{},{},{}", 'E' as u8, 'E' as usize, 'z' as u8);
        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day12 {
    fn find_endpoints(&self) -> (Point, Point){
        let mut start_x = 0;
        let mut start_y = 0;
        let mut target_x = 0;
        let mut target_y = 0;

        for y in 0..self.heightmap.len() { 
            for x in 0..self.heightmap[y].len() {
                if self.heightmap[y][x] == 'S' as u8 {
                    start_x = x;
                    start_y = y;
                } else if self.heightmap[y][x] == 'E' as u8 {
                    target_x = x;
                    target_y = y;
                }
            }
        }
        (Point(start_x, start_y), Point(target_x, target_y))
    }


    fn a_star(&self, start: Point, target: Point, ) -> Option<Vec<Point>> {
        // Copied from pseudoCode on wikipedia, plus slight rust help from `pathfinding` crate source.
        // But that is more optimised. Complate lack of borrows here in favour of copies feels so not rust

        // The set of discovered points that need to be expanded. Intially, only start is known.
        // If this becomes a BinaryHeap, it becomes faster to find 'current' below
        let mut open_set: HashSet<Point> = HashSet::new();
        open_set.insert(start);

        // For Point p, came_from[p] is the point immediately preceding it on the cheapest path from
        // start to p currently known
        let mut came_from: HashMap<Point, Point> = HashMap::new();

        // for point p, g_score[p] is the cost of the cheapest path from start to n currently known
        let mut g_score: HashMap<Point, usize> = HashMap::new();
        g_score.insert(start, 0);

        // f_score would be g_score modified by a heuristic, but we don't have one, so just ignore it

        while open_set.len() > 0 {
            let current: Point = *open_set.iter()
                .min_by_key(|&point| g_score.get(point).unwrap_or(&usize::MAX))
                .unwrap();

            //println!("Inspecting {current}");

            if current == target {
                return Some(Self::reconstruct_path(came_from, current));
            }

            open_set.remove(&current);
            for neighbour in self.neighbours(current) {
                // We would calculate weight here, but we don't have any
                //print!("  Neighbour {neighbour}: ");
                let tentative_g_score = g_score.get(&current).unwrap() + 1;
                if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, tentative_g_score);
                    // Update f_score here if we include it
                    
                    open_set.insert(neighbour);
                    //print!("inserted")
                }
                //println!();
            }
        }
        None
    }

    fn neighbours(&self, p: Point) -> Vec<Point> {
        let mut result = vec![];
        let max_height = self.get_height(p) + 1; // We can step one higher than our current height

        let max_y = self.heightmap.len() -1 ;
        let max_x = self.heightmap[0].len() -1 ;
        // We can only go at most one higher than 'mine'
        // Down (y + 1)
        if p.1 != max_y && self.get_height(p.down()) <= max_height {
            result.push(Point(p.0, p.1 + 1));
        }

        // Up (y - 1)
        if p.1 != 0 && self.get_height(p.up()) <= max_height {
            result.push(Point(p.0, p.1 - 1));
        }

        // Left (x - 1)
        if p.0 != 0 && self.get_height(p.left()) <= max_height {
            result.push(Point(p.0 - 1, p.1));
        }

        // Right (x + 1)
        if p.0 != max_x && self.get_height(p.right()) <= max_height {
            result.push(Point(p.0 + 1, p.1));
        }

        //println!("Neighbours of {p}({})", (max_height - 1) as char);
        /*
        for r in &result {
            print!("{r}({}),", self.heightmap[r.1][r.0] as char);
        }
        println!();
        */

        result
    }

    fn get_height(&self, point: Point) -> u8 {
        let mut height = self.heightmap[point.1][point.0];

        // Correct for endpoints. This is horrible lol
        if height == 'S' as u8 { height = 'a' as u8 };
        if height == 'E' as u8 { height = 'z' as u8 };

        height
    }

    fn reconstruct_path(came_from: HashMap<Point, Point>, endpoint: Point) -> Vec<Point> {
        // Following the path set out in came_from
        //println!("making path to: {endpoint}");

        let mut result: Vec<Point> = vec![];
        let mut current = endpoint;
        loop {
            result.push(current);
            match came_from.get(&current) {
                Some(&parent) => {
                    current = parent;
                    //println!("new current: {current}");
                },
                None => {
                    result.reverse();
                    return result; 
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(usize, usize);

impl Point {
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
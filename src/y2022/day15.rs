use std::{fmt::Display, collections::HashSet, ops::{RangeInclusive, RangeTo}, io::Write};

use super::super::day::Day;

const PART1_TARGET_Y: isize = 10;
//const PART1_TARGET_Y: isize = 2000000;

const PART2_MAX: isize = 20;
//const PART2_MAX: isize = 4000000;

pub struct Day15
{
    scanners: Vec<Scanner>,
}

impl Day15 {
    pub fn new() -> Day15
    {
        let input = include_str!("input15");
        let input = include_str!("input15_example");

        let scanners = input.trim().split('\n')
            .map(Scanner::from_str).collect::<Vec<Scanner>>();


        Day15 { scanners }
    }
}

impl Day for Day15 {
    fn day_name(&self) -> String { String::from("15") }
    fn answer1(&self) -> String { String::from("6425133") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {

        let target_row_beacons = self.scanners.iter().filter(|scanner| scanner.beacon_y == PART1_TARGET_Y).collect::<Vec<_>>();
        let coverage_ranges = self.scanners.iter().map(|s| s.coverage_horizontal(PART1_TARGET_Y)).collect::<Vec<_>>();
        // we have an iterator of ranges, find the total covered area....
        // This seems like a bad way to do this

        let lowest_covered = *coverage_ranges.iter().min_by_key(|r| r.start()).expect("?").start();
        let highest_covered = *coverage_ranges.iter().max_by_key(|r| r.end()).expect("?").end();

        let mut covered = 0;
        for point in lowest_covered..=highest_covered {
            // Ignore beacons
            if !target_row_beacons.iter().any(|b| b.beacon_x == point) 
            && coverage_ranges.iter().any(|r| r.contains(&point)) {
                covered += 1;
            }
        }
        
        println!("Coverage Rate: {}/{} ({}%)", covered, highest_covered - lowest_covered, ((covered * 100)/(highest_covered - lowest_covered)));

        let ans1 = covered;

        let mut ans2 = 0;
        'y_loop: for y in 0..=PART2_MAX {
            print!("\r {y} / {PART2_MAX} ({:02.2}%)", (y as f32 * 100.0) / PART2_MAX as f32);
            std::io::stdout().flush().unwrap();
            let target_row_beacons = self.scanners.iter().filter(|scanner| scanner.beacon_y == y).collect::<Vec<_>>();
            let coverage_ranges = self.scanners.iter().map(|s| s.coverage_horizontal(y)).collect::<Vec<_>>();

            for x in 0..=PART2_MAX {
                // Ignore beacons
                if !target_row_beacons.iter().any(|b| b.beacon_x == x) 
                && !coverage_ranges.iter().any(|r| r.contains(&x)) {
                    ans2 = (x * 4000000) + y;
                    break 'y_loop;
                }
            }
        }
        println!();


        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day15 {
    fn print_coverage_2(&self, coverage_target: &Scanner) {
        // Find the mins and the maxes
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for scanner in &self.scanners {
            min_x = isize::min(min_x, isize::min(scanner.x, scanner.beacon_x));
            max_x = isize::max(max_x, isize::max(scanner.x, scanner.beacon_x));
            min_y = isize::min(min_y, isize::min(scanner.y, scanner.beacon_y));
            max_y = isize::max(max_y, isize::max(scanner.y, scanner.beacon_y));
        }

        for y in min_y..=max_y {
            print!("{y:02}: ");
            for x in min_x..=max_x {
                if self.scanners.iter().any(|s| s.x == x && s.y == y) {
                    print!("S");
                } else if self.scanners.iter().any(|s| s.beacon_x == x && s.beacon_y == y) {
                    print!("B");
                } else if  coverage_target.covers(x, y){
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scanner {
    x: isize,
    y: isize,
    beacon_x: isize,
    beacon_y: isize,

    beacon_dist: isize,
}

impl Scanner {
    fn from_str(input: &str) -> Scanner {
        let parts = input.trim().split(' ').collect::<Vec<_>>();
        let x_str = parts[2];
        let y_str = parts[3];
        let bx_str = parts[8];
        let by_str = parts[9];

        let x = x_str[2..x_str.len() - 1].parse::<isize>().expect("Parse Error");
        let y = y_str[2..y_str.len() - 1].parse::<isize>().expect("Parse Error");
        let beacon_x = bx_str[2..bx_str.len() - 1].parse::<isize>().expect("Parse Error");
        let beacon_y = by_str[2..].parse::<isize>().expect("Parse Error");

        let beacon_dist = (x - beacon_x).abs() + (y - beacon_y).abs();

        Scanner { x, y, beacon_x, beacon_y, beacon_dist }
    }

    // TODO! This is very slow, try instead writing is_covered(x, y), so we can just loop through the row and see if we can reach any
    // scanners, instead of computing the entire coverage range...
    fn covers(&self, x: isize, y: isize) -> bool {
        let x_dist = (self.x - x).abs();
        let y_dist = (self.y - y).abs();
        x_dist + y_dist <= self.beacon_dist
    }

    fn coverage(&self) -> HashSet<(isize, isize)> {
        println!("Coverage for {},{}...", self.x, self.y);
        let mut result = HashSet::new();
        let beacon_distance = (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs();
        println!("Beacon distance = {beacon_distance}");

        for y in (self.y-beacon_distance)..=(self.y + beacon_distance) {
            let x_dist = beacon_distance - (self.y - y).abs(); // How many steps of 'distance' do we have left to allocate to x direction?

            for x in (self.x-x_dist)..=(self.x + x_dist) {
                result.insert((x, y));
            }
        }
        result
    }

    fn coverage_horizontal(&self, y: isize) -> RangeInclusive<isize> {
        // Simply, how far can we reach in either direction, at this y range
        let x_dist = self.beacon_dist - (self.y - y).abs(); // How many steps of 'distance' do we have left to allocate to x direction?
        (self.x-x_dist)..=(self.x + x_dist)
    }
}
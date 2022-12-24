﻿use std::{ops::RangeInclusive, time::Instant};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::super::day::Day;

//const PART1_TARGET_Y: i32 = 10;
const PART1_TARGET_Y: i32 = 2000000;

//const PART2_MAX: i32 = 20;
const PART2_MAX: i32 = 4000000;

const CHUNKS: usize = 100;
const CHUNK_SIZE: usize = PART2_MAX as usize / CHUNKS;

pub struct Day15
{
    scanners: Vec<Scanner>,
}

impl Day15 {
    #[allow(dead_code)]
    pub fn new() -> Day15
    {
        let input = include_str!("input15");
        //let input = include_str!("input15_example");

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
        let coverage_ranges = self.scanners.iter().filter_map(|s| s.coverage_horizontal(PART1_TARGET_Y)).collect::<Vec<_>>();
        //let coverage_ranges = combine_coverage_ranges(&coverage_ranges);
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

        let ans2 = (0..CHUNKS).into_par_iter().find_map_any(|chunk| {
            self.check_chunk(chunk)
        }).expect("Didn't find answer...");
        println!();


        println!("{ans1}, {ans2} {}", ans2 == 56000011);
        (ans1.to_string() , ans2.to_string())
    }
}

fn combine_coverage_ranges(coverage_ranges: &Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    let mut result = vec![];
    for range in coverage_ranges {
        // if this range isn't covered entirely by another, include it
        if !coverage_ranges.iter().any(|other| other.start() < range.start() && other.end() > range.end()) {
            result.push(range.clone());
        }
    }
    result
}

impl Day15 {
    fn check_chunk(&self, my_chunk: usize) -> Option<i32> {
        let my_start = (my_chunk * CHUNK_SIZE) as i32;
        let my_end = my_start + CHUNK_SIZE as i32;
        let before = Instant::now();
        let result = (my_start..=my_end).find_map(|y| {
            /*
            if y % 100 == 0 {
                println!("{} / {CHUNK_SIZE} ({:02.2}%)", y - my_start, ((y - my_start) as f32 * 100.0) / CHUNK_SIZE as f32);
            }
            */

            let answer = self.check_row(y);


            answer
        });
        let elapsed = before.elapsed();
        println!("chunk {my_chunk} took {:04} millis. {CHUNKS} chunks would take {} minutes", elapsed.as_millis(), (elapsed * CHUNKS as u32).as_secs() / 60);
        result
    }
    
    fn check_row(&self, check_y: i32) -> Option<i32> {
        let coverage_ranges = self.scanners.iter().filter_map(|s| s.coverage_horizontal(check_y)).collect::<Vec<_>>();
        let coverage_ranges = combine_coverage_ranges(&coverage_ranges);
        for x in 0..=PART2_MAX {
            // Ignore beacons
            if !coverage_ranges.iter().any(|r| r.contains(&x))
            && !self.scanners.iter().any(|scanner| scanner.beacon_y == check_y && scanner.beacon_x == x) {
                return Some((x as i32 * 4000000) + check_y as i32);
            }
        }
        None
    }

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scanner {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
    beacon_dist: i32,
}

impl Scanner {
    fn from_str(input: &str) -> Scanner {
        let parts = input.trim().split(' ').collect::<Vec<_>>();
        let x_str = parts[2];
        let y_str = parts[3];
        let bx_str = parts[8];
        let by_str = parts[9];

        let x = x_str[2..x_str.len() - 1].parse::<i32>().expect("Parse Error");
        let y = y_str[2..y_str.len() - 1].parse::<i32>().expect("Parse Error");
        let beacon_x = bx_str[2..bx_str.len() - 1].parse::<i32>().expect("Parse Error");
        let beacon_y = by_str[2..].parse::<i32>().expect("Parse Error");

        let beacon_dist = (x - beacon_x).abs() + (y - beacon_y).abs();

        Scanner { x, y, beacon_x, beacon_y, beacon_dist }
    }

    fn coverage_horizontal(&self, y: i32) -> Option<RangeInclusive<i32>> {
        // Simply, how far can we reach in either direction, at this y range
        let x_dist = self.beacon_dist - (self.y - y).abs(); // How many steps of 'distance' do we have left to allocate to x direction?
        if x_dist > 0 {
            Some((self.x-x_dist)..=(self.x + x_dist))
        } else {
            None
        }
    }
}
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::super::day::Day;

const PART1_TARGET_Y: i32 = 2000000;
const PART2_MAX: i32 = 4000000;
const CHUNKS: usize = 5000;

/* for test data...
const PART1_TARGET_Y: i32 = 10;
const PART2_MAX: i32 = 20;
const CHUNKS: usize = 10;
*/

const CHUNK_SIZE: usize = PART2_MAX as usize / CHUNKS;

pub struct Day15
{
    scanners: Vec<Scanner>,
}

impl Day15 {
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
    fn answer2(&self) -> String { String::from("10996191429555") }

    fn part1(&mut self) -> String {
        let coverage_ranges = self.scanners.iter().filter_map(|s| s.coverage_horizontal(PART1_TARGET_Y)).collect::<Vec<_>>();
        // we have an iterator of ranges, find the total covered area....
        // This seems like a bad way to do this

        let lowest_covered = coverage_ranges.iter().min_by_key(|r| r.0).expect("?").0;
        let highest_covered = coverage_ranges.iter().max_by_key(|r| r.1).expect("?").1;

        let covered = (lowest_covered..=highest_covered).into_par_iter().filter(|&x| 
            coverage_ranges.iter().any(|r| Self::check_cell(r, x))
        ).count();
        
        (covered - 1).to_string() // Why is this a minus 1, not minus target row beacons . len() ? not sure !
    }

    fn part2(&mut self) -> String {
        (0..CHUNKS).into_par_iter().find_map_any(|chunk| {
            self.check_chunk(chunk)
        }).expect("Didn't find answer...").to_string()
    }
}

impl Day15 {
    fn check_chunk(&self, my_chunk: usize) -> Option<usize> {
        let my_start = (my_chunk * CHUNK_SIZE) as i32;
        let my_end = my_start + CHUNK_SIZE as i32;

        (my_start..=my_end).find_map(|y| {
            self.check_row(y)
        })
    }
    
    fn check_row(&self, check_y: i32) -> Option<usize> {
        // Start with no coverage (one range of the entire row). 
        // For each scanner, we will carve out the sections of the range that it covers
        // Eventually, most rows will lose all their uncovered ranges.
        // Our target row will end up with one range, that contains one point (the one possible position)

        let mut uncovered_ranges = vec![(0, PART2_MAX)];
        for scanner in &self.scanners {
            let ours = scanner.coverage_horizontal(check_y);
            if ours.is_none() { continue; }
            let ours = ours.unwrap();

            let mut new_uncovered = vec![];
            'range_loop: for other in &uncovered_ranges {
                // ours is completely before other, no interaction
                if ours.1 < other.0 {
                    new_uncovered.push(*other);
                    continue 'range_loop;
                }

                // ours is completely after other, no interaction
                if ours.0 > other.1 {
                    new_uncovered.push(*other);
                    continue 'range_loop;
                }

                // ours contains with other completely, removing other completely
                if ours.0 <= other.0 && ours.1 >= other.1 {
                    continue 'range_loop;
                }

                // ours overlaps with other from the left, chop of the start of other
                if ours.0 <= other.0 {
                    new_uncovered.push((ours.1 + 1, other.1));
                    continue 'range_loop;
                }

                // ours overlaps other from the right, chop off the end of other
                if ours.1 >= other.1 {
                    new_uncovered.push((other.0, ours.0 - 1));
                    continue 'range_loop;
                }

                // ours is contained within other, split other into a before part and an after part
                new_uncovered.push((other.0, ours.0 - 1));
                new_uncovered.push((ours.1 + 1, other.1));
            }

            uncovered_ranges = new_uncovered;
        }

        if !uncovered_ranges.is_empty() {
            // this should be exactly one element, a one length range (x, x)
            let x = uncovered_ranges[0].0;
            Some((x as usize * 4000000) + check_y as usize)
        } else {
            None
        }
    }

    fn check_cell(range: &(i32, i32), x: i32) -> bool {
        range.0 <= x && x <= range.1
    }

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scanner {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
    beacon_dist: i32,

    min_y: i32,
    max_y: i32,
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

        let min_y = y - beacon_dist;
        let max_y = y + beacon_dist;

        Scanner { x, y, beacon_x, beacon_y, beacon_dist, min_y, max_y }
    }

    fn coverage_horizontal(&self, y: i32) -> Option<(i32, i32)> {
        // Simply, how far can we reach in either direction, at this y range
        let x_dist = self.beacon_dist - (self.y - y).abs(); // How many steps of 'distance' do we have left to allocate to x direction?
        if x_dist > 0 {
            Some((self.x-x_dist, self.x + x_dist))
        } else {
            None
        }
    }
}
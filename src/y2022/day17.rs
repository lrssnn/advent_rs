﻿use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

pub struct Day17
{
    chamber: Chamber,
}

impl Day17 {
    pub fn new() -> Day17
    {
        let input = include_str!("input17");
        //let input = include_str!("2022-17.txt"); // Uncle Scientist's
        //let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let chamber = Chamber::new(input.chars().collect());

        Day17 { chamber }
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    //fn answer1(&self) -> String { String::from("3085") }
    //fn answer2(&self) -> String { String::from("?") }
    fn answer1(&self) -> String { String::from("3068") } // Example
    //fn answer2(&self) -> String { String::from("1514285714288") } // Example
    fn answer2(&self) -> String { String::from("1582758620701") } // Uncle's

    fn part1(&mut self) -> String {
        return "".to_string();
        let mut part1 = self.chamber.clone();

        for _ in 0..2022 {
            part1.drop_one();
        }

        return part1.height().to_string();
    }

    fn part2(&mut self) -> String {
        let mut part2 = self.chamber.clone();

        // state will be: current piece number, current jet index, top 4 rows of chamber
        // if we get a repeat, then we found a cycle
        //      -- delta_height: height from previous cycle to this one
        //      -- delta_drops: how many drops were needed to get to delta_height
        //      -- offset_height: how high the tower was when the cycle began for the first time
        //      -- offset_drops: how many drops it took to get to offset_height

        let mut cycle_finder = HashMap::new();

        // map state to (height, drops)
        cycle_finder.insert((part2.piecenum, part2.jetnum, 0u64), (0usize, 0usize));

        let mut drops = 0;
        loop {
            part2.drop_one();
            drops += 1;
            let height = part2.height();
            if height < 4 {
                continue;
            }

            let shape = ((part2.rocks[height - 1] as usize) << 24)
                | ((part2.rocks[height - 2] as usize) << 16)
                | ((part2.rocks[height - 3] as usize) << 8)
                | (part2.rocks[height - 4] as usize);

            let skyline = u64::from_ne_bytes(part2.rocks[part2.rocks.len() - 8..].try_into().unwrap());

            if let Some(entry) = cycle_finder.get(&(part2.piecenum, part2.jetnum, skyline)) {
                println!("piece = {}", part2.piecenum);
                println!("jetnum = {}", part2.jetnum);
                println!("shape = {}", shape);

                println!("drops until start of loop = {}", entry.1);
                println!("height of tower when the loop started = {}", entry.0);
                let delta_height = height - entry.0;
                let delta_drops = drops - entry.1;
                println!(
                "There is an increase of {delta_height} rows for every {delta_drops} drops"
                );
                let remaining_drops = Chamber::OUCH - entry.1;
                println!("There are still {remaining_drops} left to go");

                let needed_drops = remaining_drops / delta_drops;
                let leftover_drops = remaining_drops % delta_drops;
                let integral_height = entry.0 + delta_height * needed_drops;

                println!(
                "The height will reach {integral_height} but there are still {leftover_drops} drops left"
                );

                for _ in 0..leftover_drops {
                    part2.drop_one();
                }
                let leftover_height = part2.height() - height;
                println!("After {leftover_drops} more drops, we added {leftover_height} rows");
                let answer = integral_height + leftover_height;
                if (answer == 1514534883742) { println!("Known bad answer!!!\n");}

                return (answer).to_string();
            } else {
                cycle_finder.insert((part2.piecenum, part2.jetnum, skyline), (height, drops));
            }
        }
    }
}

#[derive(Default, Clone)]
struct Chamber {
    jets: Vec<char>,
    rocks: Vec<u8>,
    piecenum: usize,
    jetnum: usize,
}

impl Chamber {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110],
        [0b0000000, 0b0001000, 0b0011100, 0b0001000],
        [0b0000000, 0b0000100, 0b0000100, 0b0011100],
        [0b0010000, 0b0010000, 0b0010000, 0b0010000],
        [0b0000000, 0b0000000, 0b0011000, 0b0011000],
    ];

    const OUCH: usize = 1_000_000_000_000;

    fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            rocks: vec![0, 0, 0, 0, 0, 0, 0],
            piecenum: 0,
            jetnum: 0,
        }
    }

    fn drop_one(&mut self) {
        let mut piece = Self::PIECES[self.piecenum];
        self.piecenum = (self.piecenum + 1) % Self::PIECES.len();

        // make room at the top for the new piece
        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            // start off by using the jet to move the piece left or right
            let jet = self.jets[self.jetnum];
            self.jetnum = (self.jetnum + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad input '{jet}'"),
            }

            // drop the piece by one if it can
            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            self.rocks[bottom] |= piece[prow];
            bottom += 1;
        }
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x40) != 0 || (self.rocks[bottom] & (piece[prow] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x01) != 0 || (self.rocks[bottom] & (piece[prow] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (self.rocks[bottom] & piece[prow]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }
}
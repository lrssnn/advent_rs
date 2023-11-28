use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};

use crate::two_dimensional::coord::Coord as CoordGeneric;

use super::super::day::Day;

const OUCH: usize = 1_000_000_000_000;
type Coord = CoordGeneric<isize>;

pub struct Day17
{
    jets: Vec<char>,
}

impl Day17 {
    pub fn new() -> Day17
    {
        let input = include_str!("input17");
        //let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        Day17 { jets: input.chars().collect() }
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    fn answer1(&self) -> String { String::from("3085") }
    fn answer2(&self) -> String { String::from("1535483870924") } // Obtained from someone else's code on my input

    fn part1(&mut self) -> String {
        simulate(2023, &self.jets).to_string()
    }

    fn part2(&mut self) -> String {
        simulate(OUCH, &self.jets).to_string()
    }

}

fn simulate(limit: usize, jets: &[char]) -> isize {
    // Port of https://topaz.github.io/paste/#XQAAAQAUBQAAAAAAAAA5G8iNzA96Qa088jTp5Ejfbji94jfZrlIdoFF1L9sFu0LGygmgp2I70u9FstDo0dRkTOWyMzphAzE8vDJhtED9mZCdlWuODec5bnjjBcH8EIFjQ025xJDs9nUPwsJk2IhYSZivONPbcP6GraZ87xCaSldzIYodcIhf86R/AFMQ7F4X2iVtESxO+dBJS2NfeqClpFXEouluAaevZXqoB10cKVo8mVjC3ZjFrlB5/QVcLboXkxBPvIC2oXxa0b6/a8orfW1g9NsS0XIlyijfvDnVI8DjnoT7Nw/Jf0fX/QI2jL3JrsVo5kQeFSnvUW5QWWFvANz0BWDL50o4g15xo+N1rJG693CW6u4X/ZdAzLj538I31hJ89KTTfTRCcG8aIZd1DrVLum9soQ1FIjqw9ZN9DMX6W/A5YHPYJOngkS26UP+zy0vnNgA6MpFF8hgPGUKotpkm1AnsIjyT4tiWmhEmcNFAtwOoSrECzLiLqLRdv5X4bkOOw78tpqEYChSiOETWtPo+OBKdb3GVnn959iB5TFInSNEZFaMe9aCF7x4Zs4JBM/ubij83/j+z+a3NFX/kgPnpFL59L+CXhCFiUwDScfutsRBgsiAVYGBFdGGYvc/PRPmBukhqU823F4gAz+OahcvRjDlBoke6viZ9UzAQGg/yqj2dNUH/G8DsQAMXahCAIWC+1kIICqRNlewUlu6acCd5v2FuvqwEL7LM03b8OE5h6mdge5zTn5MFA1xmIwjJHFGkSzbd5F1XfTmuZKqsgddaxO4olSSiKgnCPIijYSNfEYPiUTg+B7t3Ey6Fo2H/6LrF8g==
    // Tried to rename things appropriately but I don't reeeeally understand what he is doing, so difficult

    let mut rock_index = 0;
    let mut jet_index = 0;

    let rocks = vec![
        vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0), Coord::new(3, 0)],
        vec![Coord::new(1, 0), Coord::new(0, 1), Coord::new(2, 1), Coord::new(1, 2)],
        vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0), Coord::new(2, 1), Coord::new(2, 2)],
        vec![Coord::new(0, 0), Coord::new(0, 1), Coord::new(0, 2), Coord::new(0, 3)],
        vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(0, 1), Coord::new(1, 1)]
    ];

    let jets = jets.iter()
        .map(|c| *c as isize -61)
        .collect::<Vec<_>>();

    let mut tower = HashSet::new();
    let mut cache = HashMap::new();
    let mut top = 0;


    for step in 0..limit - 1  {
        let mut pos = Coord::new(2, top+4); // set start pos

        let key = (rock_index,jet_index);

        if let Vacant(e) = cache.entry(key) {
            e.insert((step, top));
        } else {
            let (s, t) = cache.get(&key).unwrap();

            let d = (OUCH - step) / (step - s);
            let m = (OUCH - step) % (step - s);

            if m == 0 {
                let answer: isize = top + (top-t) * d as isize;
                return answer;
            } 
        }

        let rock = rocks[rock_index].clone();                             // get next rock
        rock_index = (rock_index+1) % rocks.len();                     // and inc index

        loop {
            let jet = Coord::new(jets[jet_index], 0);

            jet_index = (jet_index+1) % jets.len();                   // and inc index

            if check(&tower, pos, jet, rock.clone()) { pos = pos + jet; }    // maybe move side
            let down = Coord::new(0, -1);
            if check(&tower, pos, down, rock.clone()) { pos = pos + down; }    // maybe move down
            else { break; }                             // can't move down
        }

        let result = rock.iter().map(|&r| pos + r).collect::<HashSet<_>>();
        tower.extend(result);
        let offset = match rock_index {
            0 => 1,
            1 => 0,
            2 => 2,
            3 => 2,
            4 => 3,
            _ => panic!(),
        };
        top = isize::max(top, pos.y + offset)     // compute new top
    }
    top
}

fn is_empty(tower: &HashSet<Coord>, pos: &Coord) -> bool {
    (0..7).contains(&pos.x) && 
    pos.y > 0 && 
    !tower.contains(pos)
}

fn check(tower: &HashSet<Coord>, pos: Coord, dir: Coord, rock: Vec<Coord>) -> bool {
    rock.iter().all(|&r| is_empty(tower, &(pos + dir + r)))
} 

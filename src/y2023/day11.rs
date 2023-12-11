use std::collections::HashSet;
use rayon::iter::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator};

use crate::two_dimensional::coord::Coord as GenericCoord;

type Coord = GenericCoord<i16>;

use super::super::day::Day;

pub struct Day11
{
    universe: Vec<Vec<bool>>,
    galaxies: Option<Vec<Coord>>,
    expansions: Option<(HashSet<usize>, HashSet<usize>)>,
}

impl Day11 {
    pub fn new() -> Day11
    {
        //let input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        let input = include_str!("../../input/y2023/11");

        let universe = input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();

        Day11 { universe, galaxies: None, expansions: None }
    }
}

impl Day for Day11 {
    fn day_name(&self) -> String { String::from("11") }
    fn answer1(&self) -> String { String::from("9545480") }
    fn answer2(&self) -> String { String::from("406725732046") }

    fn part1(&mut self) -> String {
        let (galaxies, (expansion_rows, expansion_cols)) = find_galaxies(&self.universe);

        self.galaxies = Some(galaxies);
        self.expansions = Some((expansion_rows, expansion_cols));

        self.total_distances(1).to_string()
    }

    fn part2(&mut self) -> String {
        self.total_distances(999_999).to_string()
    }

}

impl Day11 {
    fn total_distances(&self, expansion_factor: usize) -> usize {
        // TODO Can I get rid of these clones....
        let (expansion_rows, expansion_cols) = self.expansions.as_ref().unwrap();
        let galaxies = self.galaxies.as_ref().unwrap();

        galaxies.par_iter().enumerate().map(|(i, a)| {
            galaxies[(i + 1)..].iter().map(|b| 
                get_distance(a, b, expansion_rows, expansion_cols, expansion_factor)
            ).sum::<usize>()
        }).sum()
    }
}

#[allow(clippy::needless_range_loop)] // Suggested code looks horrendous
fn find_galaxies(universe: &Vec<Vec<bool>>) -> (Vec<Coord>, (HashSet<usize>, HashSet<usize>)) {
    let mut coords = vec![];
    let mut empty_rows: Vec<bool> = (0..universe.len()).map(|_| true).collect();
    let mut empty_cols: Vec<bool> = (0..universe[0].len()).map(|_| true).collect();

    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] {
                empty_rows[i] = false;
                empty_cols[j] = false;
                coords.push(Coord::new(j as i16, i as i16));
            }
        }
    }

    let empty_rows = empty_rows.iter().enumerate().filter(|&(_, &empty)| empty).map(|(r, _)| r).collect();
    let empty_cols = empty_cols.iter().enumerate().filter(|&(_, &empty)| empty).map(|(c, _)| c).collect();

    (coords, (empty_rows, empty_cols))
}

// TODO This should go into coord but I'm not sure how to express it generically
fn get_distance(a: &Coord, b: &Coord, expansion_rows: &HashSet<usize>, expansion_cols: &HashSet<usize>, expansion_factor: usize) -> usize {
    let x_min = a.x.min(b.x) as usize;
    let x_max = a.x.max(b.x) as usize;
    let y_min = a.y.min(b.y) as usize;
    let y_max = a.y.max(b.y) as usize;

    let row_crossings = expansion_rows.iter().filter(|&r| y_min < *r && *r < y_max ).count();
    let col_crossings = expansion_cols.iter().filter(|&c| x_min < *c && *c < x_max ).count();

    let expansion = (row_crossings + col_crossings) * expansion_factor;

    (a.x.abs_diff(b.x) + a.y.abs_diff(b.y)) as usize + expansion
}

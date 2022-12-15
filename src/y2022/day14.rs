use std::fmt::Display;

use super::super::day::Day;

pub struct Day14
{
    grid: Vec<Vec<Cell>>,
}

impl Day14 {
    pub fn new() -> Day14
    {
        let input = include_str!("input14");
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

        let grid = Cell::build_grid(input.trim());

        Day14 { grid }
    }
}

impl Day for Day14 {
    fn day_name(&self) -> String { String::from("14") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        let ans1 = 0;

        let ans2 = 0;

        //println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day14 {
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Obstacle,
    Free,
    Sand
}

impl Cell {
    fn build_grid(input: &str) -> Vec<Vec<Cell>> {
        // First we are going to find the minimum x value, so we can adjust from the big values to our vec values
        let (xs, ys): (Vec<usize>, Vec<usize>) = input.split('\n').flat_map(|line| line.split(" -> ")).map(|pair| {
            let parts = pair.split(',').collect::<Vec<_>>();
            (parts[0].parse::<usize>().expect("n"), parts[1].parse::<usize>().expect("n"))
        }).unzip();
        let min_x = *xs.iter().min().expect("No min x?");
        let max_x = *xs.iter().max().expect("No max x?");
        let max_y = *ys.iter().max().expect("No max y?");
        let width = max_x - min_x;

        let mut grid = Cell::empty_grid(width + 1, max_y + 1);

        println!("{:?}", grid);
        Cell::print_grid(&grid);

        // Now parse the input into paths
        let paths = input.split('\n').map(|line| Path::from_str(line, min_x))
            .collect::<Vec<Path>>();

        // and follow each path filling the grid with obstacles
        for path in &paths {
            println!("{path}");
            for point in path.get_full_path() { 
                println!(" {:?}", point);
                grid[point.1][point.0] = Cell::Obstacle;
            }
        }

        Cell::print_grid(&grid);

        grid
    }

    fn empty_grid(width: usize, height: usize) -> Vec<Vec<Cell>> {
        (0..height).map(|_row| {
            println!("row ");
            (0..width).map(|_col| Cell::Free).collect()
        }
        ).collect()
    }

    fn print_grid(grid: &Vec<Vec<Cell>>) {
        for row in grid {
            for cell in row {
                print!("{cell}");
            }
            println!();
        }
    }
}

struct Path {
    points: Vec<(usize, usize)>,
}

impl Path {
    fn from_str(input: &str, bias: usize) -> Path {
        let points = input.split(" -> ").map(|pair| {
            let parts = pair.split(',').collect::<Vec<_>>();
            (parts[0].parse::<usize>().expect("!") - bias, parts[1].parse::<usize>().expect("!")) 
        }).collect();
        Path {points}
    }

    fn get_full_path(&self) -> Vec<(usize, usize)> {
        let mut points = self.points.iter();
        let mut curr = points.next().expect("!");
        let mut result = vec![];
        for point in points {
            let prev = curr;
            curr = point;
            result.extend(Self::interp(*prev, *curr));
        }

        result.dedup();
        result
    }

    fn interp(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![start];
        let step = Self::find_step(start, end);
        let mut current = Self::add(start, step);
        while current != end {
            result.push(current);
            current = Self::add(current, step);
        }
        result.push(end);
        result
    }

    fn find_step(start: (usize, usize), end: (usize, usize)) -> (isize, isize) {
        // Horizontal or vertical, then positive or negative
        // We assume that we will always be given a straight line, I think...
        match (start.0.cmp(&end.0), start.1.cmp(&end.1)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => (1, 0),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => (0, 1),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => (0, -1),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => (-1, 0),
            _ => panic!("Need to fill out match arms :("),
        }
    }

    fn add(start: (usize, usize), op: (isize, isize)) -> (usize, usize) {
        ((start.0 as isize + op.0) as usize,
        (start.1 as isize + op.1) as usize)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Obstacle => write!(f, "#"),
            Cell::Free => write!(f, "."),
            Cell::Sand => write!(f, "o"),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.points[0].0, self.points[0].1).expect("!");
        for point in self.points.iter().skip(1) {
            write!(f, " -> {},{}", point.0, point.1).expect("!");
        }
        Ok(())
    }
}
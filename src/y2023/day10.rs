use std::{fmt::Display, collections::HashSet};

use crate::two_dimensional::{coord::Coord as GenericCoord, direction::Direction};

type Coord = GenericCoord<i16>;

use super::super::day::Day;

pub struct Day10
{
    map: Vec<Vec<Tile>>,
    start: (usize, usize),
    main_loop: HashSet<Coord>,
}

impl Day10 {
    pub fn new() -> Day10
    {
        //let input = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        //let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";
        //let input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";
        //let input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";
        let input = include_str!("../../input/y2023/10");

        let mut start = (0, 0);
        let map = input.lines().enumerate()
            .map(|(row, l)| l.chars().enumerate()
                .map(|(col, c)| {
                    let t = Tile::from_char(c);
                    if t.eq(&Tile::Start) { start = (col, row); }
                    t
                }).collect()).collect();

        Day10 { map, start, main_loop: HashSet::new() }
    }
}

impl Day for Day10 {
    fn day_name(&self) -> String { String::from("10") }
    fn answer1(&self) -> String { String::from("7030") }
    fn answer2(&self) -> String { String::from("285") }

    fn part1(&mut self) -> String {
        self.replace_start();
        let start = Coord::new(self.start.0 as i16, self.start.1 as i16);
        let mut neighbours = self.neighbours(start).unwrap();

        self.main_loop.insert(start);
        self.main_loop.insert(neighbours.0.0);
        self.main_loop.insert(neighbours.1.0);

        // We know for sure we are in a loop, so we should walk one step in each direction until we meet in the middle at the furthest point
        let mut distance = 1; // We already took the first step above
        while neighbours.0.0 != neighbours.1.0 {
            distance += 1;
            neighbours.0 = self.walk_from(neighbours.0.0, neighbours.0.1);
            neighbours.1 = self.walk_from(neighbours.1.0, neighbours.1.1);

            self.main_loop.insert(neighbours.0.0);
            self.main_loop.insert(neighbours.1.0);
            //println!("Step {distance}, at {} & {}", neighbours.0.0, neighbours.1.0);
        }

        distance.to_string()
    }

    fn part2(&mut self) -> String {
        (0..self.map.len()).into_iter().map(|r| self.evaluate_row(r)).sum::<usize>().to_string()
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Start,
    Vertical,
    Horizontal,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
    None,
}

impl Day10 {
    fn replace_start(&mut self) {
        let directions = self.find_start_directions(Coord::new(self.start.0 as i16, self.start.1 as i16));
        self.map[self.start.1][self.start.0] = Tile::from_directions(directions);
    }

    fn get_tile(&self, coord: Coord) -> Tile {
        self.map[coord.y as usize][coord.x as usize]
    }

    fn neighbours(&mut self, my_coord: Coord) -> Option<((Coord, Direction), (Coord, Direction))> {
        let tile = self.get_tile(my_coord);
        let offsets = self.accessible_directions(&tile).unwrap();
        Some(((my_coord + offsets.0, offsets.0), (my_coord + offsets.1, offsets.1)))
    }

    fn find_start_directions(&mut self, my_coord: Coord) -> (Direction, Direction) {
        // This is gross
        let mut first = None;
        let mut second = None;
        for checking_direction in Direction::all().iter() {
            let checking_coord = my_coord + *checking_direction;
            if checking_coord != my_coord {
                if let Some(neighbours) = self.neighbours(checking_coord) {
                    if neighbours.0.0 == my_coord || neighbours.1.0 == my_coord {
                        if first.is_none() { first = Some(checking_direction.clone()); }
                        else if second.is_none() { second = Some(checking_direction.clone()); }
                        else { panic!("Found more than two neighbours..."); }
                    }
                }
            }
        }
        
        (first.unwrap(), second.unwrap())
    }

    fn walk_from(&mut self, from_coord: Coord, last_step_direction: Direction) -> (Coord, Direction) {
        let wrong_way = last_step_direction.inverse();
        let options = self.neighbours(from_coord).unwrap();
        if options.0.1 != wrong_way {
            options.0
        } else {
            options.1
        }
    }

    fn evaluate_row(&self, row_idx: usize) -> usize {
        let row = &self.map[row_idx];

        let mut score = 0;
        let mut crossings = 0;

        for i in 0..row.len() {
             let this_coord = Coord::new(i as i16, row_idx as i16);

             if self.main_loop.contains(&this_coord) {
                if row[i].can_go_up() { crossings += 1; }
             } else {
                // When crossings is odd, we are "inside"
                score += crossings % 2;
             }
        }

        score
    }

    fn accessible_directions(&self, tile: &Tile) -> Option<(Direction, Direction)> {
        match tile {
            Tile::Vertical => Some((Direction::Up, Direction::Down)),
            Tile::Horizontal => Some((Direction::Left, Direction::Right)),
            Tile::BottomLeft => Some((Direction::Up, Direction::Right)),
            Tile::BottomRight => Some((Direction::Up, Direction::Left)),
            Tile::TopLeft => Some((Direction::Down, Direction::Right)),
            Tile::TopRight => Some((Direction::Down, Direction::Left)),
            Tile::None => None,
            Tile::Start => panic!("Failed to replace Start"),
        }
        
    }
}

impl Tile {
    fn from_char(input: char) -> Tile {
        match input {
            'S' => Self::Start,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BottomLeft,
            'J' => Self::BottomRight,
            '7' => Self::TopRight,
            'F' => Self::TopLeft,
            '.' => Self::None,
            x => panic!("Unknown Tile {x}"),
        }
    }

    fn from_directions(directions: (Direction, Direction)) -> Tile {
        match directions {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => Tile::Vertical,
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up)=> Tile::BottomRight,
            (Direction::Up, Direction::Right)|(Direction::Right, Direction::Up)  => Tile::BottomLeft,
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => Tile::TopRight,
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down)=> Tile::TopLeft,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => Tile::Horizontal,
            _ => panic!("Invalid directions"),
        }
    }
    
    fn can_go_up(&self) -> bool {
        match self {
            Tile::Start => panic!("Failed to replace start tile"),
            Tile::Vertical | Tile::BottomLeft | Tile::BottomRight => true,
            _ => false,
        }
    }

}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 
        match self {
            Self::Start => 'S',
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::BottomLeft => 'L',
            Self::BottomRight => 'J',
            Self::TopRight => '7',
            Self::TopLeft => 'F',
            Self::None => '.',
        })
    }
}
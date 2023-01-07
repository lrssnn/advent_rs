﻿use std::fmt::Display;

use super::super::day::Day;

const WIDTH: usize = 150;
const HEIGHT: usize = 200;

/*
const WIDTH: usize = 16;
const HEIGHT: usize = 12;
*/

const COL_WIDTH : usize = WIDTH / 4;
const ROW_HEIGHT: usize = HEIGHT / 3;

type Board = [[Cell; WIDTH]; HEIGHT];
pub struct Day22
{
    board: Board,
    moves: Vec<Move>,
}

impl Day22 {
    pub fn new() -> Day22
    {
        let input = include_str!("input22");
        //let input = include_str!("input22_example");

        let (board_s, moves_s) = input.split_once("\n\n").unwrap();

        let board = board_s.lines()
            .map(|l| {
                let mut line = [Cell::Skip; WIDTH];
                for(i, c) in l.chars().enumerate() {
                    line[i] = Cell::from_char(c);
                }
                line
            }).collect::<Vec<_>>().try_into().unwrap();

        let moves = Move::vec_from_str(moves_s.trim());

        Day22 { board, moves }
    }

    fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                print!("{cell}");
            }
            println!();
        }
     }
    
    fn process(&self, wrap_strategy: &dyn Fn(Position, Direction) -> (Position, Direction)) -> (Position, Direction) {
        let x = self.board[0].iter().position(|&c| c == Cell::Open).unwrap();
        let mut pos = Position {x, y: 0};
        let mut fac = Direction::East;

        for m in &self.moves {
            //print_board_location(&self.board,&pos);
            (pos, fac) = process_move(m, pos, fac, &self.board, wrap_strategy);
        }

        (pos, fac)
    }
}

fn process_move(m: &Move, mut pos: Position, mut fac: Direction, board: &Board, wrap_strategy: &dyn Fn(Position, Direction) -> (Position, Direction)) -> (Position, Direction) {
    //println!("Processing '{m}'");
    match m {
        Move::Right => (pos, fac.turn_right()),
        Move::Left => (pos, fac.turn_left()),
        Move::Steps(steps) => {
            for _step in 0..*steps {
                let new_pos = pos.take_step(fac, board, wrap_strategy);
                //println!("After {_step} steps: ");
                //print_board_location(&board, &new_pos);
                if new_pos.0 == pos {
                    break;
                }
                (pos, fac) = new_pos;
            }
            (pos, fac)
        },
    }
}

fn print_board_location(board: &Board, pos: &Position) {
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if pos.x == x && pos.y == y{
                print!("X");
            } else {
                print!("{cell}");
            }
        }
        println!();
    }
}

fn move_strategy_part_1(pos: Position, facing: Direction) -> (Position, Direction) {
    // part 1 can't change your facing, but part 2 can
    let pos = match facing {
        Direction::North => Position { x: pos.x, y: wrap_up_simple(pos)},
        Direction::South => Position { x: pos.x, y: wrap_down_simple(pos)},
        Direction::East => Position { x: wrap_right_simple(pos), y: pos.y},
        Direction::West => Position { x: wrap_left_simple(pos), y: pos.y},
    };

    (pos, facing)
}

fn wrap_right_simple(pos: Position) -> usize {
    match which_row(&pos) {
        1 => { if pos.x == face_end_x(1) { return face_start_x(1); }}, 
        2 => { if pos.x == face_end_x(4) { return face_start_x(2); }}, 
        3 => { if pos.x == face_end_x(6) { return face_start_x(5); }},
        _ => panic!("Invalid"),
    };
    return pos.x + 1;
}

fn wrap_left_simple(pos: Position) -> usize {
    match which_row(&pos) {
        1 => { if pos.x == face_start_x(1) { return face_end_x(1) }}, 
        2 => { if pos.x == face_start_x(2) { return face_end_x(4) }},
        3 => { if pos.x == face_start_x(5) { return face_end_x(6) }},
        _ => panic!("Invalid"),
    }
    return pos.x - 1;
}

fn wrap_down_simple(pos: Position) -> usize {
    match which_col(&pos) {
        1 | 2 => { if pos.y == face_end_y(2) { return face_start_y(2) }},
        3     => { if pos.y == face_end_y(5) { return face_start_y(1) }},
        4     => { if pos.y == face_end_y(6) { return face_start_y(6) }},
        _ => panic!("Invalid"),
    }

    return pos.y + 1;
}

fn wrap_up_simple(pos: Position) -> usize {
    match which_col(&pos) {
        1 | 2 => { if pos.y == face_start_y(2) { return face_end_y(2) }},
        3     => { if pos.y == face_start_y(1) { return face_end_y(5) }},
        4     => { if pos.y == face_start_y(6) { return face_end_y(6) }},
        _ => panic!("Invalid"),
    }

    return pos.y - 1;
}

fn wrap_right_cube(pos: Position, fac: Direction) -> (Position, Direction) {
    match which_row(&pos) {
        1 => { if pos.x == face_end_x(1) { 
            // more y on the 1 Face = less y on the 6 face
            return (Position {x: face_end_x(6), y: face_end_y(6) - y_on_face(&pos, 1) }, Direction::West); 
        }},
        2 => { if pos.x == face_end_x(4) { 
            // More y on the 4 Face = less x on the 6 face
            return (Position {x: face_end_x(6) - y_on_face(&pos, 4), y: face_start_y(6) }, Direction::South); 
        }},
        3 => { if pos.x == face_end_x(6) { 
            // More y on the 6 Face = less y on the 1 face
            return (Position {x: face_end_x(1), y: face_end_y(1) - y_on_face(&pos, 6)}, Direction::West); 
        }},
        _ => panic!("Invalid"),
    }

    (Position {x: pos.x + 1, y: pos.y}, fac)
}

fn wrap_left_cube(pos: Position, fac: Direction) -> (Position, Direction) {
    match which_row(&pos) {
        1 => { if pos.x == face_start_x(1) {
                // More y on the 1 face = more x on the 3 face
                return (Position {x: face_start_x(3) + y_on_face(&pos, 1), y: face_start_y(3)}, Direction::South);
            }
        },
        2 => { if pos.x == face_start_x(2) {
                // More y on 2 = less x on 6
                return (Position {x: face_end_x(6) - y_on_face(&pos, 2), y: face_end_y(6)}, Direction::North);
            }
        },
        3 => { if pos.x == face_start_x(5) {
                // More y on 5 = less x on 3
                return (Position {x: face_end_x(3) - y_on_face(&pos, 5), y: face_end_y(3)}, Direction::North);
            }
        },
        _ => panic!("Invalid"),
    }

    (Position { x: pos.x - 1, y: pos.y }, fac)
}

fn wrap_down_cube(pos: Position, fac: Direction) -> (Position, Direction) {
    match which_col(&pos) {
        1 => { if pos.y == face_end_y(2) {
            // more x on 2 = less x on 5
            return (Position {x: face_end_x(5) - x_on_face(&pos, 2), y: face_end_y(5)}, Direction::North)
        }},
        2 => { if pos.y == face_end_y(3) {
            // more x on 3 = less y on 5
            return (Position {x: face_start_x(5), y: face_end_y(5) - x_on_face(&pos, 3)}, Direction::East)
        }},
        3 => { if pos.y == face_end_y(5) {
            // more x on 5 = less x on 2
            return (Position {x: face_end_x(2) - x_on_face(&pos, 5), y: face_end_y(2)}, Direction::North)
        }},
        4 => { if pos.y == face_end_y(6) {
            // more x on 6 = less y on 2
            return (Position {x: face_start_x(2), y: face_end_y(2) - x_on_face(&pos, 6)}, Direction::East)
        }},

        _=> panic!("Invalid"),
    }


    (Position { x: pos.x, y: pos.y + 1 }, fac)
}

fn wrap_up_cube(pos: Position, fac: Direction) -> (Position, Direction) {
    match which_col(&pos) {
        1 => { if pos.y == face_start_y(2) {
            // more x on 2 = less x on 1
            return (Position {x: face_end_x(1) - x_on_face(&pos, 2), y: face_start_y(1)}, Direction::South)
        }},
        2 => { if pos.y == face_start_y(3) {
            // more x on 3 = more y on 1
            return (Position {x: face_start_x(1), y: face_start_y(1) + x_on_face(&pos, 3)}, Direction::East)
        }},
        3 => { if pos.y == face_start_y(1) {
            // more x on 1 = less x on 2
            return (Position {x: face_end_x(2) - x_on_face(&pos, 1), y: face_start_y(2)}, Direction::South)
        }},
        4 => { if pos.y == face_start_y(6) {
            // more x on 6 = less y on 4
            return (Position {x: face_end_x(4), y: face_end_y(4) - x_on_face(&pos, 6)}, Direction::West)
        }},
        _ => panic!("Invalid"),
    };

    (Position { x: pos.x, y: pos.y - 1}, fac)
}

fn which_face(pos: &Position) -> u8 {
    match (which_row(pos), which_col(pos)) {
        (1, _) => 1,
        (2, 1) => 2,
        (2, 2) => 3,
        (2, 3) => 4,
        (3, 3) => 5,
        (3, 4) => 6,
        _ => panic!("Invalid"),
    }
}

fn which_col(pos: &Position) -> u8 {
    if pos.x < COL_WIDTH { 1 }
    else if pos.x < 2 * COL_WIDTH { 2 }
    else if pos.x < 3 * COL_WIDTH { 3 }
    else { 4 }
}

fn which_row(pos: &Position) -> u8 {
    if pos.y < ROW_HEIGHT { 1 }
    else if pos.y < 2 * ROW_HEIGHT { 2 }
    else { 3 }
}


fn move_strategy_part_2(pos: Position, facing: Direction) -> (Position, Direction) {
    let result = match facing {
        Direction::North => wrap_up_cube(pos, facing),
        Direction::South => wrap_down_cube(pos, facing),
        Direction::East => wrap_right_cube(pos, facing),
        Direction::West => wrap_left_cube(pos, facing),
    };
    //println!("  Wrapped {pos} -> {} ({facing} -> {}), (face {} -> face {})", result.0, result.1, which_face(&pos), which_face(&result.0));
    result
}

fn face_start_x(face_num: u8) -> usize {
    match face_num {
        1 => 2 * COL_WIDTH,
        2 => 0 * COL_WIDTH,
        3 => 1 * COL_WIDTH,
        4 => 2 * COL_WIDTH,
        5 => 2 * COL_WIDTH,
        6 => 3 * COL_WIDTH,
        _ => panic!("Invalid"),
    }
}

fn face_end_x(face_num: u8) -> usize {
    match face_num {
        1 => 3 * COL_WIDTH - 1,
        2 => 1 * COL_WIDTH - 1,
        3 => 2 * COL_WIDTH - 1,
        4 => 3 * COL_WIDTH - 1,
        5 => 3 * COL_WIDTH - 1,
        6 => 4 * COL_WIDTH - 1,
        _ => panic!("Invalid"),
    }
}

fn face_start_y(face_num: u8) -> usize {
    match face_num {
        1 => 0 * ROW_HEIGHT,
        2 => 1 * ROW_HEIGHT,
        3 => 1 * ROW_HEIGHT,
        4 => 1 * ROW_HEIGHT,
        5 => 2 * ROW_HEIGHT,
        6 => 2 * ROW_HEIGHT,
        _ => panic!("Invalid"),
    }
}

fn face_end_y(face_num: u8) -> usize {
    match face_num {
        1 => 1 * ROW_HEIGHT - 1,
        2 => 2 * ROW_HEIGHT - 1,
        3 => 2 * ROW_HEIGHT - 1,
        4 => 2 * ROW_HEIGHT - 1,
        5 => 3 * ROW_HEIGHT - 1,
        6 => 3 * ROW_HEIGHT - 1,
        _ => panic!("Invalid"),
    }
}

fn x_on_face(pos: &Position, face_num: u8) -> usize {
    pos.x - face_start_x(face_num)
}

fn y_on_face(pos: &Position, face_num: u8) -> usize {
    pos.y - face_start_y(face_num)
}

impl Day for Day22 {
    fn day_name(&self) -> String { String::from("22") }
    fn answer1(&self) -> String { String::from("65368") }
    fn answer2(&self) -> String { String::from("Unknown") }

    fn part1(&mut self) -> String {
        //println!();
        //self.print_board();
        //println!();
        //for m in &self.moves { print!("{m} ")};
        //println!();

        let (pos, facing) = self.process(&move_strategy_part_1);
        let password = (1000 * (pos.y + 1)) + (4 * (pos.x + 1)) + facing.value();
        println!("{password}");
        password.to_string()
    }

    fn part2(&mut self) -> String {
        let (pos, facing) = self.process(&move_strategy_part_2);
        let password = (1000 * (pos.y + 1)) + (4 * (pos.x + 1)) + facing.value();
        password.to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Skip, Wall, Open
}

impl Cell {
    fn from_char(input: char) -> Cell {
        match input {
            ' ' => Cell::Skip,
            '.' => Cell::Open,
            '#' => Cell::Wall,
            _ => panic!("Invalid char"),
        }
    }
}

enum Move {
    Steps(u32), Right, Left,
}

impl Move {
    fn vec_from_str(input: &str) -> Vec<Move> {
        let mut result = vec![];

        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            result.push(
                if c == 'R' {
                    Move::Right
                } else if c == 'L' {
                    Move::Left
                } else {
                    let mut num = c.to_string();
                    while let Some(next) = chars.next_if(|e| e.is_numeric()) {
                        num += &next.to_string();
                    }
                    //println!("'{num}'");
                    Move::Steps(num.parse().unwrap()) 
                }
            );
        }

        result
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: usize, y: usize
}
impl Position {
    fn take_step(&self, fac: Direction, board: &Board, wrap_strategy: &dyn Fn(Position, Direction) -> (Position, Direction)) -> (Position, Direction) {
        let next = wrap_strategy(*self, fac);
        //println!("Next = {next} '{}'", board[next.y][next.x]);

        if board[next.0.y][next.0.x] == Cell::Wall {
            (*self, fac)
        } else {
            next
        }
    }

    // Maybe this would be cleaner if it took a Direction instead. Would clean up above
    fn wrap_step(&self, offset: (i32, i32)) -> Position {
        // Gross :)
        let result = if self.x == 0 && offset.0 == -1 {
            Position { x: WIDTH - 1, y: (self.y as i32 + offset.1) as usize }
        } else if self.x == WIDTH - 1 && offset.0 == 1 {
            Position { x: 0, y: (self.y as i32 + offset.1) as usize }
        } else if self.y == 0 && offset.1 == -1 {
            Position { x: (self.x as i32 + offset.0) as usize, y: HEIGHT - 1 }
        } else if self.y == HEIGHT - 1 && offset.1 == 1 {
            Position { x: (self.x as i32 + offset.0) as usize, y: 0 }
        } else {
            Position { x: (self.x as i32 + offset.0) as usize, y: (self.y as i32 + offset.1) as usize}
        };
        
        result
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North, South, East, West,
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn value(&self) -> usize {
        match self {
            Direction::North => 3,
            Direction::South => 1,
            Direction::East => 0,
            Direction::West => 2,
        }
    }

    pub(crate) fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::North => "N".to_string(),
            Direction::South => "S".to_string(),
            Direction::East => "E".to_string(),
            Direction::West => "W".to_string(),
        })
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Skip => ' ',
            Cell::Wall => '#',
            Cell::Open => '.',
        })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Move::Steps(i) => i.to_string(),
            Move::Right => "R".to_string(),
            Move::Left => "L".to_string(),
        })
    }
}

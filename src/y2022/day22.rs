use std::fmt::Display;

use super::super::day::Day;

/*
const WIDTH: usize = 150;
const HEIGHT: usize = 200;
*/

const WIDTH: usize = 16;
const HEIGHT: usize = 12;

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
        let input = include_str!("input22_example");

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
            (pos, fac) = process_move(m, pos, fac, &self.board, &move_strategy_part_1);
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
    let result = if pos.x == 0 && facing == Direction::West {
        Position { x: WIDTH - 1, y: pos.y }
    } else if pos.x == WIDTH - 1 && facing == Direction::East {
        Position { x: 0, y: pos.y }
    } else if pos.y == 0 && facing == Direction::North {
        Position { x: pos.x, y: HEIGHT - 1 }
    } else if pos.y == HEIGHT - 1 && facing == Direction::South {
        Position { x: pos.x, y: 0 }
    } else {
        let offset = facing.offset();
        Position { x: (pos.x as i32 + offset.0) as usize, y: (pos.y as i32 + offset.1) as usize}
    };
    
    (result, facing)
}

fn move_strategy_part_2(pos: Position, facing: Direction) -> (Position, Direction) {
    let result = if pos.x == 0 && facing == Direction::West {
        Position { x: WIDTH - 1, y: pos.y }
    } else if pos.x == WIDTH - 1 && facing == Direction::East {
        Position { x: 0, y: pos.y }
    } else if pos.y == 0 && facing == Direction::North {
        Position { x: pos.x, y: HEIGHT - 1 }
    } else if pos.y == HEIGHT - 1 && facing == Direction::South {
        Position { x: pos.x, y: 0 }
    } else {
        let offset = facing.offset();
        Position { x: (pos.x as i32 + offset.0) as usize, y: (pos.y as i32 + offset.1) as usize}
    };
    
    (result, facing)
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

        let (pos, facing) = self.process();
        let password = (1000 * (pos.y + 1)) + (4 * (pos.x + 1)) + facing.value();
        password.to_string()
    }

    fn part2(&mut self) -> String {
        let (pos, facing) = self.process();
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
        let mut now = (*self, fac);
        let mut next = wrap_strategy(*self, fac);
        //println!("Next = {next} '{}'", board[next.y][next.x]);

        loop {
            if board[next.0.y][next.0.x] == Cell::Skip {
                now = next;
                next = wrap_strategy(next.0, next.1);
                //println!("Next = {next} '{}'", board[next.y][next.x]);
            } else if board[next.0.y][next.0.x] == Cell::Wall {
                // THis is a horrible way to deal with being stopped after a wrap
                if board[now.0.y][now.0.x] == Cell::Skip {
                    return now.0.take_step(fac.reverse(), &board, wrap_strategy);
                }
                return now;
            } else {
                return next
            }
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

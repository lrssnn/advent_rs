use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up, Down, Left, Right, None
}

impl Direction {
    pub fn from_char(c: char) -> Direction {
        match c  {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown char type")
        }
    }

    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0 , 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::None => (0, 0),
        }
    }

    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![Direction::Down, Direction::Right, Direction::Up, Direction::Left, Direction::None]
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::None => '.',
        })
    }
}
use std::{ops::Add, ops::Sub, cmp::PartialEq, convert::From, fmt::Display};
use crate::two_dimensional::direction::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord<T> 
{
    pub x: T,
    pub y: T,
}

impl<T> Coord<T>
where T: TryFrom<isize> + PartialOrd + Copy,
isize: From<T>
{
    pub fn new(x: T, y: T) -> Coord<T> {
        Coord { x, y }
    }

    pub fn wrapped_add(self, rhs: Direction, valid_x: (T, T), valid_y: (T, T)) -> Coord<T> {
        let mut res = self + rhs;
        if res.x < valid_x.0 { res.x = valid_x.1;}
        if res.x > valid_x.1 { res.x = valid_x.0;}
        if res.y < valid_y.0 { res.y = valid_y.1;}
        if res.y > valid_y.1 { res.y = valid_y.0;}
        res
    }
}

impl<T: PartialEq> PartialEq<(T, T)> for Coord<T> {
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<T> Add<Direction> for Coord<T> 
where isize: From<T>, T: TryFrom<isize> {
    fn add(self, rhs: Direction) -> Self {
        self + rhs.offset()
    }

    type Output = Self;
}

impl<T> Add<(isize, isize)> for Coord<T> 
where isize: From<T>, T: TryFrom<isize> {
    fn add(self, rhs: (isize, isize)) -> Self {
        let x: T = ((Into::<isize>::into(self.x)) + rhs.0).max(0).try_into().ok().unwrap();
        let y: T = ((Into::<isize>::into(self.y)) + rhs.1).max(0).try_into().ok().unwrap();
        Coord { x, y }
    }

    type Output = Self;
}

impl<T> Sub<Coord<T>> for Coord<T>
where T: Sub<T, Output = T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Display for Coord<T> 
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
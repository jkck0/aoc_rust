use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, 1);
pub const RIGHT: Point = Point::new(1, 0);
pub const DOWN: Point = Point::new(0, -1);
pub const LEFT: Point = Point::new(-1, 0);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

// Parse a Point from a string representation like "x,y"
impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(",")
            .ok_or_else(|| ParsePointError)
            .map(|(x, y)| {
                Ok((
                    x.parse().map_err(|_| ParsePointError)?,
                    y.parse().map_err(|_| ParsePointError)?,
                ))
            })??;

        Ok(Point { x: x, y: y })
    }
}

use std::ops::{Index, IndexMut};

use crate::util::point::Point;

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Grid {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn raw(&self) -> &[T] {
        &self.data
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self.data[(point.x + point.y * self.width as i32) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.data[(point.x + point.y * self.width as i32) as usize]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGridError;
impl Grid<u8> {
    pub fn parse(s: &str) -> Result<Self, ParseGridError> {
        let lines = s.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();

        for i in 1..lines.len() {
            if lines[i].len() != width {
                return Err(ParseGridError);
            }
        }

        let data = lines.join("").bytes().collect();
        Ok(Grid {
            width,
            height,
            data,
        })
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Grid {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
        }
    }
}

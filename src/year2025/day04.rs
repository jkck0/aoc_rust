use crate::util::{
    grid::Grid,
    point::{DOWN, LEFT, Point, RIGHT, UP},
};

pub fn parse(data: &str) -> Grid<u8> {
    Grid::parse(data).unwrap()
}

fn available_directions(width: usize, height: usize, x: usize, y: usize) -> Vec<Point> {
    let mut directions = Vec::with_capacity(8);

    let up = y < height - 1;
    let right = x < width - 1;
    let down = y > 0;
    let left = x > 0;

    // cardinal directions
    if up {
        directions.push(UP)
    }
    if right {
        directions.push(RIGHT)
    }
    if down {
        directions.push(DOWN)
    }
    if left {
        directions.push(LEFT)
    }

    // diagonals
    if up && left {
        directions.push(UP + LEFT)
    }
    if up && right {
        directions.push(UP + RIGHT)
    }
    if down && right {
        directions.push(DOWN + RIGHT)
    }
    if down && left {
        directions.push(DOWN + LEFT)
    }

    directions
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    let mut accessible = 0;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = Point::new(x as i32, y as i32);
            if grid[pos] != b'@' {
                continue;
            }

            let mut neighbours = 0;
            for dir in available_directions(grid.width(), grid.height(), x, y) {
                if grid[pos + dir] == b'@' {
                    neighbours += 1;
                }
            }

            if neighbours < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    let mut grid = grid.clone();
    let mut accessible = 0;

    loop {
        let mut changed = 0;
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let pos = Point::new(x as i32, y as i32);
                if grid[pos] != b'@' {
                    continue;
                }

                let mut neighbours = 0;
                for dir in available_directions(grid.width(), grid.height(), x, y) {
                    if grid[pos + dir] == b'@' {
                        neighbours += 1;
                    }
                }

                if neighbours < 4 {
                    grid[pos] = b'.';
                    changed += 1;
                }
            }
        }

        accessible += changed;
        if changed == 0 {
            break;
        }
    }

    accessible
}

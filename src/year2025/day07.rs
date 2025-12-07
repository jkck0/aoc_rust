use std::collections::HashMap;

use crate::util::{
    grid::Grid,
    point::{LEFT, Point, RIGHT},
};

pub fn parse(data: &str) -> Grid<u8> {
    Grid::parse(data).unwrap()
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    let start_pos = grid.position(b'S').unwrap();
    let mut beams = vec![start_pos.x];

    let mut splits = 0;
    for y in 1..grid.height() {
        let mut next_beams = vec![];
        for beam in beams {
            if grid[Point::new(beam, y as i32)] == b'^' {
                splits += 1;

                if beam > 0 && next_beams.iter().find(|&&x| x == beam - 1).is_none() {
                    next_beams.push(beam - 1);
                }
                if beam < (grid.width() - 1) as i32
                    && next_beams.iter().find(|&&x| x == beam + 1).is_none()
                {
                    next_beams.push(beam + 1);
                }
            } else if next_beams.iter().find(|&&x| x == beam).is_none() {
                next_beams.push(beam);
            }
        }

        beams = next_beams;
    }

    splits
}

fn num_timelines(grid: &Grid<u8>, start_pos: Point, split_cache: &mut HashMap<Point, u64>) -> u64 {
    let mut pos = start_pos;
    while pos.y < grid.height() as i32 && grid[pos] != b'^' {
        pos.y += 1;
    }
    if pos.y == grid.height() as i32 {
        return 1;
    }

    if let Some(timelines) = split_cache.get(&pos) {
        return *timelines;
    }

    let mut timelines = 0;
    if pos.x > 0 {
        timelines += num_timelines(grid, pos + LEFT, split_cache);
    }
    if pos.x < (grid.width() - 1) as i32 {
        timelines += num_timelines(grid, pos + RIGHT, split_cache);
    }
    split_cache.insert(pos, timelines);

    timelines
}

pub fn part2(grid: &Grid<u8>) -> u64 {
    let start_pos = grid.position(b'S').unwrap();
    let mut cache = HashMap::new();
    num_timelines(grid, start_pos, &mut cache)
}

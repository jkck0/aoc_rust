use std::collections::HashSet;

use crate::util::point::*;

pub fn parse(data: &str) -> Vec<Point> {
    data.chars()
        .map(|c| match c {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1(input: &[Point]) -> usize {
    let mut positions = HashSet::with_capacity(input.len());
    let mut santa_pos = ORIGIN;
    positions.insert(santa_pos);

    for &point in input {
        santa_pos += point;
        positions.insert(santa_pos);
    }

    positions.len()
}

pub fn part2(input: &[Point]) -> usize {
    let mut positions = HashSet::with_capacity(input.len());
    let mut santa_pos = ORIGIN;
    let mut robot_pos = ORIGIN;
    positions.insert(ORIGIN);

    for (i, &point) in input.iter().enumerate() {
        if i.is_multiple_of(2) {
            santa_pos += point;
            positions.insert(santa_pos);
        } else {
            robot_pos += point;
            positions.insert(robot_pos);
        }
    }

    positions.len()
}

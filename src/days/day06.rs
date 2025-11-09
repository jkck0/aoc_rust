use std::{cmp::max, str::FromStr};

use crate::util::{
    grid::Grid,
    point::{ParsePointError, Point},
};

#[derive(Debug)]
enum InstructionType {
    Toggle,
    TurnOff,
    TurnOn,
}

#[derive(Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    start: Point,
    end: Point,
}

#[derive(Debug)]
pub enum ParseInstructionError {
    SyntaxError,
    ParsePointError(ParsePointError),
}

impl From<ParsePointError> for ParseInstructionError {
    fn from(err: ParsePointError) -> Self {
        ParseInstructionError::ParsePointError(err)
    }
}

// Parse the instruction type and start and end points from a string with the format:
// (toggle|turn off|turn on) (\d,\d) through (\d,\d)
impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let first = words.next().ok_or(Self::Err::SyntaxError)?;
        let instruction_type = {
            if first == "toggle" {
                Ok(InstructionType::Toggle)
            } else if first == "turn" {
                match words.next() {
                    Some("off") => Ok(InstructionType::TurnOff),
                    Some("on") => Ok(InstructionType::TurnOn),
                    _ => Err(Self::Err::SyntaxError),
                }
            } else {
                Err(Self::Err::SyntaxError)
            }
        }?;

        let start = words.next().ok_or(Self::Err::SyntaxError)?.parse()?;

        if words.next().is_none_or(|s| s != "through") {
            return Err(Self::Err::SyntaxError);
        }

        let end = words.next().ok_or(Self::Err::SyntaxError)?.parse()?;

        Ok(Instruction {
            instruction_type,
            start,
            end,
        })
    }
}

pub fn parse(data: &str) -> Vec<Instruction> {
    data.lines().map(|l| l.parse().unwrap()).collect()
}

fn apply_instruction1(grid: &mut Grid<bool>, instruction: &Instruction) {
    for x in instruction.start.x..=instruction.end.x {
        for y in instruction.start.y..=instruction.end.y {
            let point = Point::new(x, y);
            match instruction.instruction_type {
                InstructionType::TurnOff => grid[point] = false,
                InstructionType::TurnOn => grid[point] = true,
                InstructionType::Toggle => grid[point] = !grid[point],
            }
        }
    }
}

fn apply_instruction2(grid: &mut Grid<u32>, instruction: &Instruction) {
    for x in instruction.start.x..=instruction.end.x {
        for y in instruction.start.y..=instruction.end.y {
            let point = Point::new(x, y);
            match instruction.instruction_type {
                InstructionType::TurnOff => grid[point] = max(grid[point] as i32 - 1, 0) as u32,
                InstructionType::TurnOn => grid[point] = max(grid[point] as i32 + 1, 0) as u32,
                InstructionType::Toggle => grid[point] = max(grid[point] as i32 + 2, 0) as u32,
            }
        }
    }
}

pub fn part1(instructions: &[Instruction]) -> u32 {
    let mut grid = Grid::new(1000, 1000, false);
    for instruction in instructions {
        apply_instruction1(&mut grid, instruction);
    }

    grid.raw().iter().map(|&b| b as u32).sum()
}

pub fn part2(instructions: &[Instruction]) -> u32 {
    let mut grid = Grid::new(1000, 1000, 0);
    for instruction in instructions {
        apply_instruction2(&mut grid, instruction);
    }

    grid.raw().iter().sum()
}

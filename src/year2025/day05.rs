use std::{
    cmp::{max, min},
    num::ParseIntError,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    fn contains(&self, id: ID) -> bool {
        self.start <= id.0 && id.0 <= self.end
    }

    fn overlaps(&self, other: IDRange) -> bool {
        other.contains(ID(self.start)) || other.contains(ID(self.end))
    }

    fn num_ids(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl Add for IDRange {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        IDRange {
            start: min(self.start, rhs.start),
            end: max(self.end, rhs.end),
        }
    }
}

impl AddAssign for IDRange {
    fn add_assign(&mut self, rhs: Self) {
        self.start = min(self.start, rhs.start);
        self.end = max(self.end, rhs.end);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ID(u64);

impl FromStr for ID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

type Input = (Vec<IDRange>, Vec<ID>);

pub fn parse(data: &str) -> Input {
    let mut ranges = vec![];
    let mut ids = vec![];

    let mut lines = data.lines();
    while let Some(line) = lines.next()
        && line != ""
    {
        ranges.push(
            line.split_once("-")
                .map(|(start, end)| IDRange {
                    start: start.parse().unwrap(),
                    end: end.parse().unwrap(),
                })
                .unwrap(),
        )
    }

    for line in lines {
        ids.push(line.parse().unwrap())
    }

    (ranges, ids)
}

pub fn part1(input: &Input) -> usize {
    let (ranges, ids) = input;
    ids.iter()
        .filter(|&&id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

pub fn part2(input: &Input) -> u64 {
    let (ranges, _) = input;
    let mut ranges = ranges.clone();
    ranges.sort_unstable_by(|r1, r2| r1.start.cmp(&r2.start));

    let mut consolidated = vec![];
    let mut ranges = ranges.into_iter().peekable();
    while let Some(range) = ranges.next() {
        let mut consolidated_range = range;
        while let Some(next_range) = ranges.peek()
            && next_range.overlaps(consolidated_range)
        {
            consolidated_range += ranges.next().unwrap();
        }

        consolidated.push(consolidated_range);
    }

    consolidated.into_iter().map(|r| r.num_ids()).sum()
}

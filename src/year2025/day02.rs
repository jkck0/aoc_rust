use std::fmt::Display;

pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn invalid_ids1(&self) -> InvalidIDs1<'_> {
        InvalidIDs1 {
            range: self,
            lower_bound: self.start,
        }
    }

    pub fn invalid_ids2(&self) -> InvalidIDs2<'_> {
        InvalidIDs2 {
            range: self,
            lower_bound: self.start,
        }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

pub struct InvalidIDs1<'a> {
    range: &'a Range,
    lower_bound: u64,
}

impl<'a> Iterator for InvalidIDs1<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lower_bound > self.range.end {
            return None;
        }

        let mut x = self.lower_bound;
        // Invalid IDs always have an even number of digits
        let digits = num_digits(x);
        if !digits.is_multiple_of(2) {
            // Find the next highest number with even number of digits
            x = 10u64.pow(digits);
        }

        let mut mask = 10u64.pow(digits / 2);
        let bottom = x % mask;
        let mut top = x / mask;

        // If the number was a power of 10 before, it ends up
        // with an extra digit in the top part and the answer is
        // wrong. This fixes it. idk
        if num_digits(top) != digits / 2 {
            top /= 10;
            mask *= 10;
        }

        if bottom < top {
            x = top + top * mask;
        } else if bottom > top {
            x = (top + 1) + (top + 1) * mask;
        }

        self.lower_bound = x + 1;
        if x > self.range.end { None } else { Some(x) }
    }
}

pub struct InvalidIDs2<'a> {
    range: &'a Range,
    lower_bound: u64,
}

impl<'a> Iterator for InvalidIDs2<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        for x in self.lower_bound..=self.range.end {
            let s = format!("{}", x);
            let digits = s.len() as u32;

            // Check all possible group sizes for an invalid ID
            if factors(digits)
                .into_iter()
                .map(|size| is_repeated(&s, size))
                .any(|b| b)
            {
                self.lower_bound = x + 1;
                return Some(x);
            }
        }

        None
    }
}

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn factors(n: u32) -> Vec<u32> {
    (1..n).filter(|&x| n.is_multiple_of(x)).collect()
}

// Returns if the string is repeated groups of n characters
fn is_repeated(s: &str, n: u32) -> bool {
    let (first, mut rest) = s.split_at(n as usize);
    let mut cur;
    while rest.len() > 0 {
        (cur, rest) = rest.split_at(n as usize);
        if cur != first {
            return false;
        }
    }

    true
}

pub fn parse(data: &str) -> Vec<Range> {
    data.strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|s| {
            s.split_once("-")
                .map(|(start, end)| Range {
                    start: start.parse().unwrap(),
                    end: end.parse().unwrap(),
                })
                .unwrap()
        })
        .collect()
}

pub fn part1(input: &[Range]) -> u64 {
    input.iter().map(|r| r.invalid_ids1()).flatten().sum()
}

pub fn part2(input: &[Range]) -> u64 {
    input.iter().map(|r| r.invalid_ids2()).flatten().sum()
}

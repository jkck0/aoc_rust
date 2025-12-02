use std::fmt::Display;

pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn invalid_ids(&self) -> InvalidIDs<'_> {
        InvalidIDs {
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

pub struct InvalidIDs<'a> {
    range: &'a Range,
    lower_bound: u64,
}

impl<'a> Iterator for InvalidIDs<'a> {
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

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
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
    input.iter().map(|r| r.invalid_ids()).flatten().sum()
}

pub fn part2(input: &[Range]) -> u64 {
    todo!()
}

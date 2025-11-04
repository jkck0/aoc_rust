pub fn parse(data: &str) -> Vec<i32> {
    data.chars().map(|c| if c == '(' { 1 } else { -1 }).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> usize {
    let mut floor = 0;
    input.iter()
        .position(|c| {
            floor += c;
            floor < 0
        })
        .map(|i| i + 1)
        .unwrap()
}

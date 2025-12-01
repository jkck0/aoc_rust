use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
struct Problem {
    day: u32,
    func: fn(&str) -> (String, String),
}

fn main() {
    let day = args().nth(1).and_then(|day| day.parse().ok());

    let problems = problems()
        .into_iter()
        .filter(|p| day.is_none() || p.day == day.unwrap());

    for problem in problems {
        let Problem { day, func } = problem;
        let path = format!("input/day{day:02}.txt");
        let Ok(data) = read_to_string(path) else {
            println!("Failed opening input for day {day}");
            continue;
        };

        let (part1, part2) = func(&data);
        println!("Day {day}");
        println!("Part 1: {part1}");
        println!("Part 2: {part2}");
        println!()
    }
}

// parse out the number from a day module
// `day01` -> 1
fn parse_day_num(day: &str) -> u32 {
    let mut chars = day.bytes();
    let mut n: u32 = loop {
        let char = chars
            .next()
            .unwrap_or_else(|| panic!("cannot parse day number from module `{day}`"));
        if char >= b'0' && char <= b'9' {
            break (char - b'0').into();
        }
    };

    for char in chars {
        if char >= b'0' && char <= b'9' {
            let digit: u32 = (char - b'0').into();
            n = n * 10 + digit;
        } else {
            break;
        }
    }

    n
}

macro_rules! gen_problems {
    ($($day:tt),*) => {
        fn problems() -> Vec<Problem> {
            vec![$(
                Problem {
                    day: parse_day_num(stringify!($day)),
                    func: |data: &str| {
                        use aoc2015::days::$day::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();

                        (part1, part2)
                    },
                }
            ,)*]
        }
    }
}

gen_problems!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10
);

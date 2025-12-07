use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
struct Problem {
    year: u32,
    day: u32,
    func: fn(&str) -> (String, String),
}

fn main() {
    let mut args = args();
    let year = args.nth(1).and_then(|year| year.parse().ok());
    let day = args.next().and_then(|day| day.parse().ok());

    let problems = vec![year2015(), year2025()];

    let problems = problems
        .into_iter()
        .flatten()
        .filter(|p| year.is_none() || p.year == year.unwrap())
        .filter(|p| day.is_none() || p.day == day.unwrap());

    for problem in problems {
        let Problem { year, day, func } = problem;
        let path = format!("input/{year}/day{day:02}.txt");
        let Ok(data) = read_to_string(path) else {
            println!("Failed opening input for year {year} day {day}");
            continue;
        };

        let (part1, part2) = func(&data);
        println!("Day {day}");
        println!("Part 1: {part1}");
        println!("Part 2: {part2}");
        println!()
    }
}

// parse out the number from a day or year module
// `day01` -> 1 or `year2015` -> 2015
fn parse_mod_num(module: &str) -> u32 {
    let mut chars = module.bytes();
    let mut n: u32 = loop {
        let char = chars
            .next()
            .unwrap_or_else(|| panic!("cannot parse day number from module `{module}`"));
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
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Problem> {
            vec![$(
                Problem {
                    year: parse_mod_num(stringify!($year)),
                    day: parse_mod_num(stringify!($day)),
                    func: |data: &str| {
                        use aoc_rust::$year::$day::*;

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
    year2015
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10
);

gen_problems!(
    year2025
    day01, day02, day03, day04, day05, day06
);

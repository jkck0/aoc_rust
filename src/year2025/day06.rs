use std::cmp::max;

#[derive(Clone, Copy, Debug)]
enum Operation {
    None,
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
pub struct Problem1 {
    operation: Operation,
    nums: Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct Problem2<'a> {
    operation: Operation,
    nums: Vec<&'a str>,
}

fn parse_part1(data: &str) -> Vec<Problem1> {
    let mut problems = vec![
        Problem1 {
            operation: Operation::None,
            nums: vec![],
        };
        data.lines()
            .nth(0)
            .map(|l| l.split_ascii_whitespace().count())
            .unwrap()
    ];
    for words in data.lines().map(|l| l.split_ascii_whitespace()) {
        for (i, word) in words.enumerate() {
            if let Ok(num) = word.parse::<u64>() {
                problems[i].nums.push(num);
            } else {
                let operation = match word.chars().nth(0).unwrap() {
                    '+' => Operation::Add,
                    '*' => Operation::Multiply,
                    _ => unreachable!(),
                };
                problems[i].operation = operation;
            }
        }
    }

    problems
}

fn parse_part2<'a>(data: &'a str) -> Vec<Problem2<'a>> {
    let num_problems = data
        .lines()
        .nth(0)
        .map(|l| l.split_ascii_whitespace().count())
        .unwrap();
    let num_lines = data.lines().count();

    // store the max width of any number in a given problem,
    // to know how many chars to take later
    let mut sum_widths = vec![0; num_problems];
    for words in data
        .lines()
        .take(num_lines - 1)
        .map(|l| l.split_ascii_whitespace())
    {
        for (i, word) in words.enumerate() {
            sum_widths[i] = max(sum_widths[i], word.len());
        }
    }

    let mut problems = vec![
        Problem2 {
            operation: Operation::None,
            nums: vec![],
        };
        num_problems
    ];

    for mut line in data.lines().take(num_lines - 1) {
        for (i, width) in sum_widths.iter().enumerate() {
            let num = &line[0..*width];
            (_, line) = line.split_at_checked(width + 1).unwrap_or(("", ""));
            problems[i].nums.push(num);
        }
    }
    for (i, word) in data
        .lines()
        .nth(num_lines - 1)
        .map(|l| l.split_ascii_whitespace().enumerate())
        .unwrap()
    {
        problems[i].operation = match word.chars().nth(0).unwrap() {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => unreachable!(),
        };
    }

    problems
}

pub fn parse<'a>(data: &'a str) -> (Vec<Problem1>, Vec<Problem2<'a>>) {
    (parse_part1(data), parse_part2(data))
}

pub fn part1(input: &(Vec<Problem1>, Vec<Problem2>)) -> u64 {
    let (input, _) = input;
    input
        .iter()
        .map(|p| match p.operation {
            Operation::Add => p.nums.iter().sum::<u64>(),
            Operation::Multiply => p.nums.iter().product::<u64>(),
            Operation::None => unreachable!(),
        })
        .sum()
}

pub fn part2(input: &(Vec<Problem1>, Vec<Problem2>)) -> u64 {
    let (_, input) = input;
    input
        .iter()
        .map(|p| {
            let sum_width = p.nums[0].len();
            let mut nums = Vec::with_capacity(sum_width);

            for i in (0..sum_width).rev() {
                let mut num = 0;
                for num_str in &p.nums {
                    if let Some(c) = num_str.chars().nth(i)
                        && c != ' '
                    {
                        num *= 10;
                        num += c.to_digit(10).unwrap() as u64;
                    }
                }

                nums.push(num);
            }

            match p.operation {
                Operation::Add => nums.iter().sum::<u64>(),
                Operation::Multiply => nums.iter().product::<u64>(),
                Operation::None => unreachable!(),
            }
        })
        .sum()
}

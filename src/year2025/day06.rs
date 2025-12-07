#[derive(Clone, Copy, Debug)]
enum Operation {
    None,
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
pub struct Problem {
    operation: Operation,
    nums: Vec<u64>,
}

pub fn parse(data: &str) -> Vec<Problem> {
    let mut problems = vec![
        Problem {
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

pub fn part1(input: &[Problem]) -> u64 {
    input
        .iter()
        .map(|p| match p.operation {
            Operation::Add => p.nums.iter().sum::<u64>(),
            Operation::Multiply => p.nums.iter().product::<u64>(),
            Operation::None => unreachable!(),
        })
        .sum()
}

pub fn part2(input: &[Problem]) -> u64 {
    todo!()
}

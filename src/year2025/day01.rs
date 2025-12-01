#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Turn {
    direction: Direction,
    amount: u32,
}

pub fn parse(data: &str) -> Vec<Turn> {
    data.lines()
        .map(|l| {
            let mut chars = l.chars();
            let direction = match chars.next().unwrap() {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            };

            let mut amount = 0;
            for c in chars {
                if !c.is_ascii_digit() {
                    break;
                }
                amount *= 10;
                amount += c.to_digit(10).unwrap()
            }

            Turn { direction, amount }
        })
        .collect()
}

pub fn part1(input: &[Turn]) -> u32 {
    let mut dial = 50;
    let mut zeroes = 0;

    for t in input {
        let mult = match t.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        dial = (dial + (t.amount as i32) * mult) % 100;
        if dial == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

pub fn part2(input: &[Turn]) -> u32 {
    let mut dial = 50;
    let mut zeroes = 0;

    for t in input {
        let mult = match t.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        for _ in 0..t.amount {
            dial = (dial + mult) % 100;
            if dial == 0 {
                zeroes += 1;
            }
        }
    }

    zeroes
}

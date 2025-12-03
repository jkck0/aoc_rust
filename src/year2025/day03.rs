pub fn parse(data: &str) -> Vec<&str> {
    data.lines().collect()
}

pub fn part1(banks: &[&str]) -> u32 {
    banks
        .into_iter()
        .map(|&bank| {
            let (first_idx, first_val) = bank
                .chars()
                .rev() // .max() returns the last instance of the max value, so iterate backwards
                .skip(1)
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(i, c)| (bank.len() - 2 - i, c.to_digit(10).unwrap()))
                .unwrap();

            let second_val = bank
                .chars()
                .skip(first_idx + 1)
                .max()
                .map(|c| c.to_digit(10).unwrap())
                .unwrap();

            first_val * 10 + second_val
        })
        .sum()
}

pub fn part2(banks: &[&str]) -> u64 {
    banks
        .into_iter()
        .map(|&bank| {
            let mut joltage = 0;
            let mut used_chars = 0;

            for i in 0..12 {
                let mut digit = 0;
                let mut digit_idx = 0;

                for idx in used_chars..bank.len() - (11 - i) {
                    let c = bank
                        .chars()
                        .nth(idx)
                        .map(|c| c.to_digit(10))
                        .flatten()
                        .unwrap();

                    if c > digit {
                        digit = c;
                        digit_idx = idx;
                    }
                }

                used_chars = digit_idx + 1;
                joltage *= 10;
                joltage += digit as u64;
            }

            joltage
        })
        .sum()
}

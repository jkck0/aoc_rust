use md5;

pub fn parse(data: &str) -> &str {
    data
}

fn count_zeroes(hash: &str) -> usize {
    hash.chars().take_while(|c| *c == '0').count()
}

fn find_hash(key: &str, zeroes: usize) -> u32 {
    let mut x = 1;
    loop {
        let hash = format!("{:x}", md5::compute(key.to_owned() + &x.to_string()));
        if count_zeroes(&hash) >= zeroes {
            break;
        }

        x += 1;
    }

    x
}

pub fn part1(input: &str) -> u32 {
    find_hash(input, 5)
}

pub fn part2(input: &str) -> u32 {
    find_hash(input, 6)
}

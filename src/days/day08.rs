pub fn parse(data: &str) -> Vec<&str> {
    data.lines().collect()
}

fn memory_size(s: &str) -> usize {
    let mut inner = s
        .strip_prefix("\"")
        .map(|s| s.strip_suffix("\"").unwrap())
        .unwrap()
        .chars();

    let mut memory_chars = 0;
    while let Some(c) = inner.next() {
        if c == '\\'
            && let Some(next) = inner.next()
        {
            if next == 'x' {
                // discard the hexadecimal digits
                inner.next();
                inner.next();
            }
        }
        memory_chars += 1;
    }

    s.len() - memory_chars
}

fn code_size(s: &str) -> usize {
    // include the surrounding quotes
    let mut encoded_chars = 2;

    for c in s.chars() {
        if c == '\\' || c == '"' {
            encoded_chars += 1;
        }
        encoded_chars += 1
    }

    encoded_chars - s.len()
}

pub fn part1(strings: &[&str]) -> usize {
    strings.iter().map(|&s| memory_size(s)).sum()
}

pub fn part2(strings: &[&str]) -> usize {
    strings.iter().map(|&s| code_size(s)).sum()
}

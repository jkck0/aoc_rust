pub fn parse(data: &str) -> &str {
    data.strip_suffix("\n").unwrap_or(data)
}

fn look_and_say(input: &str) -> String {
    // Double the input length is the upper bound for how much the string can grow.
    // "123" -> "111213"
    let mut out = String::with_capacity(input.len() * 2);

    let mut bytes = input.bytes().peekable();
    while let Some(byte) = bytes.next() {
        let mut n = 1;
        while let Some(&b) = bytes.peek()
            && b == byte
        {
            bytes.next();
            n += 1;
        }
        out += &format!("{}{}", n, byte as char);
    }

    out
}

// TODO: convert String to Vec<u8> to cut down on memory use
pub fn part1(input: &str) -> usize {
    let mut s = input.to_owned();

    for _ in 0..40 {
        s = look_and_say(&s);
    }
    s.len()
}

pub fn part2(input: &str) -> usize {
    let mut s = input.to_owned();

    for _ in 0..50 {
        s = look_and_say(&s);
    }
    s.len()
}

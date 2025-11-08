fn is_nice1(s: &str) -> bool {
    let mut vowels = 0;
    let mut double_letter = false;

    let s_bytes = s.as_bytes();
    for i in 0..s_bytes.len() {
        let c = s_bytes[i];

        if let Some(&c2) = s_bytes.get(i + 1) {
            // early return for an illegal substring
            if c == b'a' && c2 == b'b'
                || c == b'c' && c2 == b'd'
                || c == b'p' && c2 == b'q'
                || c == b'x' && c2 == b'y'
            {
                return false;
            }

            if c == c2 {
                double_letter = true;
            }
        }

        if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
            vowels += 1;
        }
    }

    vowels >= 3 && double_letter
}

fn is_nice2(s: &str) -> bool {
    // Stores the counts of all bigrams. Each character can only have 26 possible values,
    // so 26 * 26 = 676 bytes is enough to store all the counts.
    // With a bigram of 2 characters c1 and c2, we can index the bigrams array by:
    // - converting each character to a number [0, 26) with (c - 1) % 32
    // - using those numbers to index the array: idx = c1 * 26 + c2
    let mut bigrams = [0u8; 26 * 26];
    let mut has_repeated = false;

    let s_bytes = s.as_bytes();

    // The purpose of can_reuse_bigram is to allow not double-counting "aaa", while
    // still correctly counting "aaaa" as 2
    let mut can_reuse_bigram = false;
    let mut last_bigram = "".as_bytes();
    for bigram in s_bytes.windows(2) {
        // if the first character of the last bigram equals the last character
        // of the current bigram, then we have the sandwich situation "xyx"
        if let Some(&c) = last_bigram.get(0) {
            if bigram[1] == c {
                has_repeated = true;
            }
        }

        if bigram != last_bigram || can_reuse_bigram {
            bigrams[bigram_to_idx(bigram)] += 1;
            can_reuse_bigram = false;
        } else {
            can_reuse_bigram = true;
        }
        last_bigram = bigram;
    }

    has_repeated && bigrams.iter().any(|&x| x >= 2)
}

fn char_to_int(c: u8) -> usize {
    ((c - 1) % 32) as usize
}

fn bigram_to_idx(bigram: &[u8]) -> usize {
    assert_eq!(bigram.len(), 2);
    (char_to_int(bigram[0]) * 26 + char_to_int(bigram[1])) as usize
}

pub fn parse(data: &str) -> Vec<&str> {
    data.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input.iter().filter(|&&s| is_nice1(s)).count() as u32
}

pub fn part2(input: &[&str]) -> u32 {
    input.iter().filter(|&&s| is_nice2(s)).count() as u32
}

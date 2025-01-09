fn three_vowels(s: &str) -> bool {
    s.bytes().zip(s.bytes().skip(1)).any(|(a, b)| a == b)
}

fn twice_in_a_row(s: &str) -> bool {
    s.bytes()
        .filter(|b| *b == b'a' || *b == b'e' || *b == b'u' || *b == b'i' || *b == b'o')
        .count()
        >= 3
}

fn no_bad_strings(s: &str) -> bool {
    !s.bytes().zip(s.bytes().skip(1)).any(|(a, b)| {
        a == b'a' && b == b'b'
            || a == b'c' && b == b'd'
            || a == b'p' && b == b'q'
            || a == b'x' && b == b'y'
    })
}

fn is_nice(s: &str) -> bool {
    three_vowels(s) && twice_in_a_row(s) && no_bad_strings(s)
}

fn pair_appears_twice(s: &str) -> bool {
    (0..s.len() - 3).any(|i| (i + 2..s.len() - 1).any(|j| s[i..i + 2] == s[j..j + 2]))
}

fn letter_repeats_spaced(s: &str) -> bool {
    s.bytes().zip(s.bytes().skip(2)).any(|(a, b)| a == b)
}

fn is_nice_better(s: &str) -> bool {
    pair_appears_twice(s) && letter_repeats_spaced(s)
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_nice(line))
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_nice_better(line))
        .count()
        .to_string()
}

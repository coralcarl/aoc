fn three_vowels(s: &str) -> bool {
    const VOWELS: [u8; 5] = [b'a', b'e', b'o', b'i', b'u'];
    s.bytes().filter(|b| VOWELS.contains(b)).count() >= 3
}

fn twice_in_a_row(s: &str) -> bool {
    s.as_bytes().windows(2).any(|v| v[0] == v[1])
}

fn no_bad_strings(s: &str) -> bool {
    const AB: [u8; 2] = [b'a', b'b'];
    const CD: [u8; 2] = [b'c', b'd'];
    const PQ: [u8; 2] = [b'p', b'q'];
    const XY: [u8; 2] = [b'x', b'y'];
    !s.as_bytes()
        .windows(2)
        .any(|v| v == AB || v == CD || v == PQ || v == XY)
}

fn is_nice(s: &str) -> bool {
    three_vowels(s) && twice_in_a_row(s) && no_bad_strings(s)
}

fn pair_appears_twice(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .enumerate()
        .any(|(i, a)| s.as_bytes()[i + 2..].windows(2).any(|b| a == b))
}

fn letter_repeats_spaced(s: &str) -> bool {
    s.as_bytes().windows(3).any(|v| v[0] == v[2])
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

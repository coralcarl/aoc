#![allow(unused_variables)]

use std::collections::HashMap;

fn digit_count(mut num: u64) -> u64 {
    let mut digit_count = 0;
    while num > 0 {
        num /= 10;
        digit_count += 1;
    }
    digit_count
}

fn blink(stone: u64, seconds: usize, visited: &mut HashMap<(u64, usize), usize>) -> usize {
    if seconds == 0 {
        return 1;
    }

    if let Some(result) = visited.get(&(stone, seconds)) {
        return *result;
    }

    let result = if stone == 0 {
        blink(1, seconds - 1, visited)
    } else {
        let digit_count = digit_count(stone);
        if digit_count % 2 == 0 {
            let p = 10_u64.pow(digit_count as u32 / 2);
            blink(stone % p, seconds - 1, visited) + blink(stone / p, seconds - 1, visited)
        } else {
            blink(stone * 2024, seconds - 1, visited)
        }
    };

    visited.insert((stone, seconds), result);
    result
}

fn blink_stones(stones: Vec<u64>, seconds: usize) -> usize {
    let mut visited = HashMap::new();
    stones
        .iter()
        .map(|&stone| blink(stone, seconds, &mut visited))
        .sum()
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> String {
    blink_stones(parse(input), 25).to_string()
}

#[allow(unreachable_code)]
pub fn part2(input: &str) -> String {
    blink_stones(parse(input), 75).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "125 17";
        assert_eq!(part1(&input), "55312");
    }
}

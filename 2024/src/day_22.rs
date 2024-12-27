#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};

struct Secret(u64);

impl Secret {
    fn next(&mut self) {
        self.0 = ((self.0 << 6) ^ self.0) & 0b1111_1111_1111_1111_1111_1111;
        self.0 = ((self.0 >> 5) ^ self.0) & 0b1111_1111_1111_1111_1111_1111;
        self.0 = ((self.0 << 11) ^ self.0) & 0b1111_1111_1111_1111_1111_1111;
    }
}

fn parse(input: &str) -> Vec<Secret> {
    input
        .lines()
        .map(|line| Secret(line.parse::<u64>().unwrap()))
        .collect()
}

fn generate_seqs_and_prices(secrets: &mut [Secret]) -> (Vec<Vec<i32>>, Vec<Vec<u64>>) {
    let mut differences = Vec::with_capacity(secrets.len());
    let mut prices = Vec::with_capacity(secrets.len());
    for (i, secret) in secrets.iter_mut().enumerate() {
        differences.push(Vec::with_capacity(2000));
        prices.push(Vec::with_capacity(2000));
        for _ in 0..2000 {
            let last = secret.0 % 10;
            secret.next();
            let new = secret.0 % 10;
            prices[i].push(new);
            differences[i].push(new as i32 - last as i32);
        }
    }
    (differences, prices)
}

pub fn part1(input: &str) -> String {
    let mut secrets = parse(input);
    for _ in 0..2000 {
        for secret in secrets.iter_mut() {
            secret.next();
        }
    }

    secrets.iter().map(|s| s.0).sum::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut secrets = parse(input);
    let mut results: HashMap<&[i32], u64> = HashMap::new();

    let (differences_all, prices_all) = generate_seqs_and_prices(&mut secrets);

    for (differences, prices) in differences_all.iter().zip(prices_all) {
        let mut seen = HashSet::new();
        for (sequence, price) in differences.windows(4).zip(prices.iter().skip(3)) {
            if seen.contains(sequence) {
                continue;
            }
            seen.insert(sequence);
            *results.entry(sequence).or_default() += price;
        }
    }

    results.values().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "1
10
100
2024";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "37327623");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "23");
    }
}

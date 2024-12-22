#![allow(unused_variables)]

use std::collections::HashMap;

fn possible<'a>(
    design: &'a str,
    towels: &[&str],
    previous_designs: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(score) = previous_designs.get(design) {
        *score
    } else {
        let score = towels
            .iter()
            .map(|towel| {
                if design.starts_with(towel) {
                    possible(&design[towel.len()..], towels, previous_designs)
                } else {
                    0
                }
            })
            .sum::<u64>();
        previous_designs.insert(design, score);
        score
    }
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut line = input.lines();
    let towels = line.next().unwrap().split(", ").collect();
    line.next();
    let designs = line.collect();
    (towels, designs)
}

pub fn part1(input: &str) -> String {
    let (towels, designs) = parse(input);
    designs
        .into_iter()
        .filter(|design| possible(design, &towels, &mut HashMap::new()) > 0)
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (towels, designs) = parse(input);
    designs
        .into_iter()
        .map(|design| possible(design, &towels, &mut HashMap::new()))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "16");
    }
}

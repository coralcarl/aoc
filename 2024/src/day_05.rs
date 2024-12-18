#![allow(unused_variables)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

type Rules<'a> = HashMap<&'a str, Vec<&'a str>>;
type Updates<'a> = Vec<Vec<&'a str>>;

fn parse(input: &str) -> (Rules, Updates) {
    let mut lines = input.lines();

    let mut rules = Rules::new();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let left = &line[..2];
        let right = &line[3..];

        rules.entry(&left).or_default();
        let page = rules.entry(&right).or_default();
        page.push(left);
    }

    let mut updates = Updates::new();

    for line in lines {
        updates.push(line.split(',').collect());
    }
    (rules, updates)
}

fn is_valid(update: &Vec<&str>, rules: &Rules) -> bool {
    let mut invalid: HashSet<&str> = HashSet::new();

    for page in update {
        if invalid.contains(page) {
            return false;
        }
        invalid.extend(&rules[page]);
    }
    true
}

fn cmp_pages(left: &str, right: &str, rules: &Rules) -> Ordering {
    if rules[right].contains(&left) {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

pub fn part1(input: &str) -> String {
    let (rules, updates) = parse(&input);

    updates
        .iter()
        .filter(|update| is_valid(update, &rules))
        .map(|update| update[update.len() / 2].parse::<u64>().unwrap())
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (rules, mut updates) = parse(&input);

    updates
        .iter_mut()
        .filter(|update| !is_valid(update, &rules))
        .map(|update| {
            update.sort_unstable_by(|a, b| cmp_pages(a, b, &rules));
            update[update.len() / 2].parse::<u64>().unwrap()
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13
13|0

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(&input), "143");
        assert_eq!(part2(&input), "123");
    }
}

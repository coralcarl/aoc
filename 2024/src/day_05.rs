use std::cmp::Ordering;

use aoclib::solution::Solution;

type Rule = Vec<Option<bool>>;
type Update = Vec<usize>;

fn parse(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut lines = input.lines();

    let mut rules = vec![vec![None; 100]; 100];

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let left = unsafe { line[..2].parse::<usize>().unwrap_unchecked() };
        let right = unsafe { line[3..].parse::<usize>().unwrap_unchecked() };

        rules[left][right] = Some(true);
        rules[right][left] = Some(false);
    }

    let updates: Vec<Update> = lines
        .map(|line| {
            (0..line.len())
                .step_by(3)
                .map(|i| unsafe { line[i..i + 2].parse::<usize>().unwrap_unchecked() })
                .collect()
        })
        .collect();

    (rules, updates)
}

fn is_sorted(update: &Update, rules: &[Rule]) -> bool {
    update.iter().enumerate().all(|(i, a)| {
        update
            .iter()
            .skip(i)
            .all(|b| rules[*a][*b].is_none_or(|v| v == true))
    })
}

fn cmp_pages(left: usize, right: usize, rules: &[Rule]) -> Ordering {
    if rules[left][right].is_none_or(|v| v == true) {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

pub fn part1(input: &str) -> Solution {
    let (rules, updates) = parse(&input);

    Solution::Usize(
        updates
            .iter()
            .filter_map(|update| is_sorted(update, &rules).then_some(update[update.len() / 2]))
            .sum::<usize>(),
    )
}

pub fn part2(input: &str) -> Solution {
    let (rules, mut updates) = parse(&input);

    Solution::Usize(
        updates
            .iter_mut()
            .filter(|update| !is_sorted(update, &rules))
            .map(|update| {
                update.sort_unstable_by(|a, b| cmp_pages(*a, *b, &rules));
                update[update.len() / 2]
            })
            .sum::<usize>(),
    )
}

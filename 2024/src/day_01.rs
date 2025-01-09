use std::collections::HashMap;

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        unsafe {
            let (x, y) = line.split_once("   ").unwrap_unchecked();
            list1.push(x.parse().unwrap_unchecked());
            list2.push(y.parse().unwrap_unchecked());
        }
    }

    (list1, list2)
}

pub fn part1(input: &str) -> String {
    let mut lists = parse(&input);
    lists.0.sort();
    lists.1.sort();

    (0..lists.0.len())
        .map(|i| lists.0[i].abs_diff(lists.1[i]))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (list1, list2) = parse(&input);

    let mut counts: HashMap<usize, usize> = HashMap::new();

    for item in list2 {
        *counts.entry(item).or_default() += item;
    }

    list1
        .into_iter()
        .map(|item| *counts.entry(item).or_default())
        .sum::<usize>()
        .to_string()
}

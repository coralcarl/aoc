#![allow(unused_variables)]

use std::collections::HashMap;

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lists: (Vec<u64>, Vec<u64>) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let (x, y) = line.split_once("   ").expect("couldn't split line");
        lists.0.push(x.parse().expect("NaN: '{x}'"));
        lists.1.push(y.parse().expect("NaN: '{y}'"));
    }

    lists
}

pub fn part1(input: &str) -> u64{
    let mut lists = parse(&input);
    lists.0.sort();
    lists.1.sort();

    let mut distance = 0;

    for (x, y) in lists.0.iter().zip(lists.1.iter()) {
        distance += x.abs_diff(*y);
    }

    distance
}

pub fn part2(input: &str) -> u64 {
    let lists = parse(&input);

    let mut frequency_table: HashMap<u64, u64> = HashMap::new();

    for num in lists.1 {
        *frequency_table.entry(num).or_default() += 1;
    }

    let mut similarity = 0;

    for num in lists.0 {
        similarity += num * *frequency_table.entry(num).or_default();
    }

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 31);
    }
}

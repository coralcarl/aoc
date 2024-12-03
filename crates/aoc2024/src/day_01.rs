use std::collections::HashMap;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut lists: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let (x, y) = line.split_once("   ").expect("couldn't split line");
        lists.0.push(x.parse::<u32>().expect("NaN: '{x}'"));
        lists.1.push(y.parse::<u32>().expect("NaN: '{y}'"));
    }

    lists
}

pub fn part1(input: &str) -> String {
    let mut lists = parse(&input);
    lists.0.sort();
    lists.1.sort();

    let mut distance = 0u32;

    for (x, y) in lists.0.iter().zip(lists.1.iter()) {
        distance += x.abs_diff(*y);
    }

    distance.to_string()
}

pub fn part2(input: &str) -> String {
    let lists = parse(&input);

    let mut frequency_table: HashMap<u32, u32> = HashMap::new();

    for num in lists.1 {
        *frequency_table.entry(num).or_default() += 1;
    }

    let mut similarity = 0u32;

    for num in lists.0 {
        similarity += num * *frequency_table.entry(num).or_default();
    }

    similarity.to_string()
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
        assert_eq!(part1(&input), "11");
        assert_eq!(part2(&input), "31");
    }
}

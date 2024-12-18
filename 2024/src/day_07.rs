#![allow(unused_variables)]

type Equation = (u64, Vec<u64>);

fn parse(input: &str) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        equations.push((
            result.parse().unwrap(),
            numbers.split(' ').map(|num| num.parse().unwrap()).collect(),
        ));
    }

    equations
}

fn solve_eq(result: u64, current: u64, remaining: &[u64], concat: bool) -> u64 {
    if remaining.len() == 0 && current == result {
        return result;
    }
    if current > result || remaining.len() == 0 {
        return 0;
    }

    if result == solve_eq(result, current * remaining[0], &remaining[1..], concat)
        || result == solve_eq(result, current + remaining[0], &remaining[1..], concat)
    {
        return result;
    }

    if concat
        && result
            == solve_eq(
                result,
                (current.to_string() + &remaining[0].to_string())
                    .parse::<u64>()
                    .unwrap(),
                &remaining[1..],
                concat,
            )
    {
        return result;
    }
    0
}

pub fn part1(input: &str) -> String {
    let equations = parse(&input);

    equations
        .iter()
        .map(|(result, numbers)| solve_eq(*result, numbers[0], &numbers[1..], false))
        .sum::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    let equations = parse(&input);

    equations
        .iter()
        .map(|(result, numbers)| solve_eq(*result, numbers[0], &numbers[1..], true))
        .sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(&input), "3749");
        assert_eq!(part2(&input), "11387");
    }
}

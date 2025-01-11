use aoclib::solution::Solution;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split([':', ' '])
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn digits(mut num: usize) -> usize {
    let mut d = 0;
    while num > 0 {
        d += 1;
        num /= 10;
    }
    d
}

fn ends_with(lhs: usize, rhs: usize) -> bool {
    (lhs - rhs) % 10_usize.pow(digits(rhs) as u32) == 0
}

fn valid_eq(result: usize, operands: &[usize], concat: bool) -> bool {
    let last = operands[operands.len() - 1];
    let remaining = &operands[..operands.len() - 1];

    if remaining.is_empty() {
        return result == last;
    }

    let quot = result / last;
    let rem = result % last;

    rem == 0 && valid_eq(quot, remaining, concat)
        || concat
            && ends_with(result, last)
            && valid_eq(
                result / 10_usize.pow(digits(last) as u32),
                remaining,
                concat,
            )
        || valid_eq(result - last, remaining, concat)
}

pub fn part1(input: &str) -> Solution {
    Solution::Usize(
        parse(&input)
            .iter()
            .filter_map(|numbers| valid_eq(numbers[0], &numbers[1..], false).then_some(numbers[0]))
            .sum::<usize>(),
    )
}

pub fn part2(input: &str) -> Solution {
    Solution::Usize(
        parse(&input)
            .iter()
            .filter_map(|numbers| valid_eq(numbers[0], &numbers[1..], true).then_some(numbers[0]))
            .sum::<usize>(),
    )
}

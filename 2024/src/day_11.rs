#![allow(unused_variables)]

fn digit_count(mut num: u64) -> u64 {
    let mut digit_count = 0;
    while num > 0 {
        num /= 10;
        digit_count += 1;
    }
    digit_count
}

fn blink(stones: &mut Vec<u64>) {
    for i in 0..stones.len() {
        if stones[i] == 0 {
            stones[i] = 1;
        } else {
            let digit_count = digit_count(stones[i]);
            if digit_count % 2 == 0 {
                let p = 10_u64.pow(digit_count as u32 / 2);
                stones.push(stones[i] % p);
                stones[i] /= p;
            } else {
                stones[i] *= 2024;
            }
        }
    }
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> String {
    let mut stones = parse(input);
    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.len().to_string()
}

#[allow(unreachable_code)]
pub fn part2(input: &str) -> String {
    let mut stones = parse(input);
    for _ in 0..75 {
        blink(&mut stones);
    }
    stones.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "125 17";
        assert_eq!(part1(&input), "55312");
        assert_eq!(part2(&input), "0");
    }
}

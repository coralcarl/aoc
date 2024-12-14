#![allow(unused_variables)]

fn blink(mut stones: Vec<u64>, count: usize) -> Vec<u64> {
    for _ in 0..count {
        let mut new_stones = Vec::new();
        for stone in &stones {
            if *stone == 0 {
                new_stones.push(1);
            } else {
                let stone_length = stone.to_string().len();
                if stone_length & 1 == 0 {
                    let num = 10_u64.pow(stone_length as u32 / 2);
                    new_stones.push(stone / num);
                    new_stones.push(stone % num);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        println!("{}", stones.len());
        stones = new_stones;
    }
    stones
}

pub fn part1(input: &str) -> u64 {
    blink(
        input
            .trim()
            .split(' ')
            .map(|num| num.parse::<u64>().unwrap())
            .collect(),
        25,
    )
    .len() as u64
}

pub fn part2(input: &str) -> u64 {
    blink(
        input
            .trim()
            .split(' ')
            .map(|num| num.parse::<u64>().unwrap())
            .collect(),
        75,
    )
    .len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "125 17";
        assert_eq!(part1(&input), 55312);
        assert_eq!(part2(&input), 0);
    }
}

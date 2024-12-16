#![allow(unused_variables)]

struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl ClawMachine {
    fn fewest_tokens_per_prize(&self) -> u64 {
        let dividend = self.px * self.by - self.py * self.bx;
        let divisor = self.ax * self.by - self.ay * self.bx;
        let a = dividend / divisor;
        if dividend != a * divisor {
            return 0;
        }
        let dividend = self.px * self.ay - self.py * self.ax;
        let divisor = self.bx * self.ay - self.by * self.ax;
        let b = dividend / divisor;
        if dividend != b * divisor {
            return 0;
        }
        (a * 3 + b) as u64
    }
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let mut arcade = Vec::new();
    for block in input.split("\n\n") {
        let mut rows = block.split('\n');
        let mut a = rows.next().unwrap()[12..]
            .split(", Y+")
            .map(|s| s.parse::<i64>().unwrap());
        let mut b = rows.next().unwrap()[12..]
            .split(", Y+")
            .map(|s| s.parse::<i64>().unwrap());
        let mut p = rows.next().unwrap()[9..]
            .split(", Y=")
            .map(|s| s.parse::<i64>().unwrap());
        arcade.push(ClawMachine {
            ax: a.next().unwrap(),
            ay: a.next().unwrap(),
            bx: b.next().unwrap(),
            by: b.next().unwrap(),
            px: p.next().unwrap(),
            py: p.next().unwrap(),
        });
    }
    arcade
}

pub fn part1(input: &str) -> u64 {
    parse(&input)
        .iter()
        .map(|arcade| arcade.fewest_tokens_per_prize())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(&input)
        .iter_mut()
        .map(|arcade| {
            arcade.px += 10000000000000;
            arcade.py += 10000000000000;
            arcade.fewest_tokens_per_prize()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), 0);
    }
}

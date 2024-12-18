#![allow(unused_variables)]

use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn score(grid: &Vec<Vec<u8>>, x: usize, y: usize, reached: &mut HashSet<(usize, usize)>) -> u64 {
    let current = grid[y][x];
    if current == 9 {
        if reached.contains(&(x, y)) {
            return 0;
        } else {
            reached.insert((x, y));
            return 1;
        }
    }

    [
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
    .into_iter()
    .filter(|(x, y)| x < &grid[0].len() && y < &grid.len() && grid[*y][*x] == current + 1)
    .map(|(x, y)| score(&grid, x, y, reached))
    .sum::<u64>()
}

fn rating(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u64 {
    let current = grid[y][x];
    if current == 9 {
        return 1;
    }

    [
        (x + 1, y),
        (x, y + 1),
        (x.wrapping_sub(1), y),
        (x, y.wrapping_sub(1)),
    ]
    .into_iter()
    .filter(|(x, y)| x < &grid[0].len() && y < &grid.len() && grid[*y][*x] == current + 1)
    .map(|(x, y)| rating(&grid, x, y))
    .sum::<u64>()
}

pub fn part1(input: &str) -> String {
    let grid = parse(&input);
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, value)| {
                    if *value == 0 {
                        return score(&grid, x, y, &mut HashSet::new());
                    } else {
                        return 0;
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse(&input);
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, value)| {
                    if *value == 0 {
                        return rating(&grid, x, y);
                    } else {
                        return 0;
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(&input), "36");
        assert_eq!(part2(&input), "81");
    }
}

#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};

struct City {
    frequencies: HashMap<char, Vec<(i32, i32)>>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> City {
    let mut width = 0;
    let mut height = 0;
    let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        height += 1;

        if width == 0 {
            width = line.len();
        }

        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            frequencies
                .entry(c)
                .or_default()
                .push((col as i32, row as i32));
        }
    }

    City {
        width,
        height,
        frequencies,
    }
}

pub fn part1(input: &str) -> u64 {
    let City {
        frequencies,
        width,
        height,
    } = parse(&input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antennas in frequencies.values() {
        for (x0, y0) in antennas.iter() {
            for (x1, y1) in antennas.iter() {
                if (x0, y0) == (x1, y1) {
                    continue;
                }
                antinodes.insert((-x1, -y0));
                antinodes.insert((2 * x1 - x0, 2 * y1 - y0));
            }
        }
    }

    antinodes
        .iter()
        .filter(|(x, y)| (*x as usize) < width && (*y as usize) < height)
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let City {
        frequencies,
        width,
        height,
    } = parse(&input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antennas in frequencies.values() {
        for (x0, y0) in antennas.iter() {
            for (x1, y1) in antennas.iter() {
                if (x0, y0) == (x1, y1) {
                    continue;
                }
                let (xd, yd) = (x1 - x0, y1 - y0);

                let mut x = *x0;
                let mut y = *y0;

                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    antinodes.insert((x, y));
                    x -= xd;
                    y -= yd;
                }
                let mut x = *x1;
                let mut y = *y1;

                while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    antinodes.insert((x, y));
                    x += xd;
                    y += yd;
                }
            }
        }
    }

    antinodes.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(&input), 14);
        assert_eq!(part2(&input), 34);
    }
}

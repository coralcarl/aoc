#![allow(unused_variables)]

use aoclib::geometry::Point;
use std::collections::HashSet;

fn map_region(
    garden: &mut Vec<Vec<char>>,
    label: char,
    plot: Point<usize>,
) -> HashSet<Point<usize>> {
    let mut region = HashSet::new();
    region.insert(plot);

    for neighbor in plot.neighbors() {
        if neighbor[1] >= garden.len() || neighbor[0] >= garden[0].len() {
            continue;
        }
        if garden[neighbor[1]][neighbor[0]] == label {
            garden[neighbor[1]][neighbor[0]] = '.';
            region.extend(map_region(garden, label, neighbor));
        }
    }
    region
}

pub fn part1(input: &str) -> String {
    let mut garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut regions = Vec::new();

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if garden[y][x] != '.' {
                let label = garden[y][x];
                garden[y][x] = '.';
                regions.push(map_region(&mut garden, label, Point::new([x, y])));
            }
        }
    }

    let mut price = 0;
    for region in &regions {
        let mut perimiter = 0;
        for plot in region {
            for neighbor in plot.neighbors() {
                if !region.contains(&neighbor) {
                    perimiter += 1;
                }
            }
        }
        price += perimiter * region.len();
    }

    price.to_string()
}

pub fn part2(input: &str) -> String {
    0.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(&input), "140");
        assert_eq!(part2(&input), "0");
    }
}

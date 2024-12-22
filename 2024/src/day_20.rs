#![allow(unused_variables)]

fn find_path(grid: &Grid) -> Vec<(usize, usize)> {
    let mut path = vec![grid.start];
    let mut current = grid.start;
    let mut prev = (0, 0);

    loop {
        let neighbors = [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ];

        for next in neighbors {
            if grid.walls[next.1][next.0] || next == prev {
                continue;
            }

            path.push(next);

            if next == grid.end {
                return path;
            }

            prev = current;
            current = next;

            break;
        }
    }
}

fn find_cheats(path: Vec<(usize, usize)>, threshold: usize, cheat_duration: usize) -> usize {
    let mut cheats = 0;
    for i in 0..path.len() - 1 {
        for j in i + 1..path.len() {
            let dist = path[i].0.abs_diff(path[j].0) + path[i].1.abs_diff(path[j].1);
            if dist <= cheat_duration && (j - i) > dist && (j - i) >= threshold + dist {
                cheats += 1;
            }
        }
    }

    cheats
}

struct Grid {
    walls: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn from(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut walls = Vec::new();

        for (y, line) in input.lines().enumerate() {
            walls.push(vec![false; width]);
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => walls[y][x] = true,
                    'S' => start = (x, y),
                    'E' => end = (x, y),
                    '.' => (),
                    _ => panic!(),
                }
            }
        }

        Self { walls, start, end }
    }
}

pub fn part1(input: &str) -> String {
    find_cheats(find_path(&Grid::from(input)), 100, 2).to_string()
}

pub fn part2(input: &str) -> String {
    find_cheats(find_path(&Grid::from(input)), 100, 20).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        let result = find_cheats(find_path(&Grid::from(&EXAMPLE1)), 20, 2).to_string();
        assert_eq!(result, "5");
    }
}

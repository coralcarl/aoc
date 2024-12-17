#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};

struct Maze {
    walls: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

fn parse(input: &str) -> Maze {
    let mut walls = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let width = input.find('\n').unwrap() - 2;
    let height = input.len() / (width + 2) - 2;

    let mut x = 0;
    let mut y = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                x = 0;
                y += 1;
            }
            '#' if x > 0 && y > 0 && x <= width && y <= height => {
                walls.insert((x, y));
            }
            'S' => start = (x, y),
            'E' => end = (x, y),
            '#' | '.' => (),
            _ => panic!("unknown character '{c}'"),
        }
        if c != '\n' {
            x += 1;
        }
    }

    Maze {
        walls,
        start,
        end,
        height,
        width,
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn required_rotations(&self, other: &Self) -> u64 {
        if self == other {
            return 0;
        }
        if *self == Direction::North && *other == Direction::South
            || *self == Direction::South && *other == Direction::North
            || *self == Direction::West && *other == Direction::East
            || *self == Direction::East && *other == Direction::West
        {
            return 2;
        }
        1
    }

    fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

fn find_min_path(
    maze: &Maze,
    position: (usize, usize),
    score: u64,
    direction: Direction,
    done: &mut HashMap<((usize, usize), Direction), u64>,
) -> u64 {
    if position == maze.end {
        return score;
    }

    // check if already visited with better score
    if let Some(best_score) = done.get(&(position, direction)) {
        if score >= *best_score {
            return u64::MAX;
        }
    }
    // check if already visted rotation with better score
    for dir in Direction::all() {
        if dir == direction {
            continue;
        }
        if let Some(best_score) = done.get(&(position, dir)) {
            if score > *best_score + direction.required_rotations(&dir) * 1000 {
                return u64::MAX;
            }
        }
    }

    // update visited score
    done.entry((position, direction))
        .and_modify(|prev_score| *prev_score = score)
        .or_insert(score);

    let next_position = match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
        Direction::East => (position.0 + 1, position.1),
    };

    [
        (next_position, direction, score + 1),
        (position, direction.cw(), score + 1000),
        (position, direction.ccw(), score + 1000),
    ]
    .into_iter()
    .map(|(p, d, s)| {
        if p.0 == 0 || p.1 == 0 || p.0 > maze.width || p.1 > maze.height || maze.walls.contains(&p)
        {
            return u64::MAX;
        }

        find_min_path(maze, p, s, d, done)
    })
    .min()
    .unwrap()
}

pub fn part1(input: &str) -> u64 {
    let maze = parse(&input);
    find_min_path(&maze, maze.start, 0, Direction::East, &mut HashMap::new())
}

pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), 7036);
        assert_eq!(part1(&EXAMPLE2), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), 0);
    }
}

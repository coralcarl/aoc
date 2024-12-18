#![allow(unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

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

use Direction::*;
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}


impl<T> Index<Direction> for Vec<T> {
    type Output = usize;

    fn index(&self, direction: Direction) -> &Self::Output {
        match direction {
            Up => &0,
            Right => &1,
            Down => &2,
            Left => &3,
        }
    }
}

impl Direction {
    fn cw(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn ccw(&self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn required_rotations(&self, other: &Self) -> u64 {
        if self == other {
            return 0;
        }
        if *self == Up && *other == Down
            || *self == Down && *other == Up
            || *self == Left && *other == Right
            || *self == Right && *other == Left
        {
            return 2;
        }
        1
    }

    fn all() -> [Direction; 4] {
        [Up, Right, Down, Left]
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
    for dir in all() {
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
        Up => (position.0, position.1 - 1),
        Down => (position.0, position.1 + 1),
        Left => (position.0 - 1, position.1),
        Right => (position.0 + 1, position.1),
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

pub fn part1(input: &str) -> String {
    let maze = parse(&input);
    find_min_path(&maze, maze.start, 0, Right, &mut HashMap::new()).to_string()
}

pub fn part2(input: &str) -> String {
    "todo".to_string()
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
        assert_eq!(part1(&EXAMPLE1), "7036");
        assert_eq!(part1(&EXAMPLE2), "11048");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "0");
    }
}

#![allow(unused_variables)]

use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(symbol: char) -> Direction {
        match symbol {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Unknown symbol: {}", symbol.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    direction: Direction,
    position: Point,
}

impl Guard {
    fn new(direction: Direction, location: Point) -> Self {
        Guard {
            direction,
            position: location,
        }
    }

    fn move_forward(&mut self, obstacles: &HashSet<Point>) {
        let new_location = match self.direction {
            Direction::North => Point::new(self.position.x, self.position.y - 1),
            Direction::East => Point::new(self.position.x + 1, self.position.y),
            Direction::South => Point::new(self.position.x, self.position.y + 1),
            Direction::West => Point::new(self.position.x - 1, self.position.y),
        };

        if obstacles.contains(&new_location) {
            self.turn_right();
        } else {
            self.position = new_location;
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone)]
struct Lab {
    guard: Guard,
    width: usize,
    height: usize,
    obstacles: HashSet<Point>,
}

fn parse(input: &str) -> Lab {
    let mut guard: Option<Guard> = None;
    let mut width = 0;
    let mut height = 0;
    let mut obstacles: HashSet<Point> = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        if height < row {
            height = row;
        }

        for (col, value) in line.chars().enumerate() {
            if width < col {
                width = col;
            }

            match value {
                '#' => {
                    obstacles.insert(Point {
                        x: col as i32,
                        y: row as i32,
                    });
                }
                '^' | '>' | 'v' | '<' => {
                    guard = Some(Guard::new(
                        Direction::from(value),
                        Point::new(col as i32, row as i32),
                    ));
                }
                '.' => (),
                _ => panic!("Unknown value: {}", value.to_string()),
            }
        }
    }

    Lab {
        guard: guard.unwrap(),
        width: width + 1,
        height: height + 1,
        obstacles,
    }
}

pub fn part1(input: &str) -> u64 {
    let Lab {
        mut guard,
        width,
        height,
        obstacles,
    } = parse(&input);

    let mut visited: HashSet<Point> = HashSet::from([guard.position]);
    loop {
        guard.move_forward(&obstacles);
        if guard.position.x < 0
            || guard.position.y < 0
            || guard.position.x == width as i32
            || guard.position.y == height as i32
        {
            return visited.len() as u64;
        }

        visited.insert(guard.position);
    }
}

pub fn part2(input: &str) -> u64 {
    let lab = parse(&input);
    let Lab {
        mut guard,
        width,
        height,
        obstacles,
    } = lab.clone();

    let mut visited: HashSet<Point> = HashSet::from([guard.position]);
    loop {
        guard.move_forward(&obstacles);
        if guard.position.x < 0
            || guard.position.y < 0
            || guard.position.x == width as i32
            || guard.position.y == height as i32
        {
            break;
        }

        visited.insert(guard.position);
    }

    let mut valid = 0;

    for pos in visited {
        let Lab {
            mut guard,
            width,
            height,
            mut obstacles,
        } = lab.clone();
        obstacles.insert(pos);
        let mut visited: HashSet<(Point, Direction)> =
            HashSet::from([(guard.position, guard.direction)]);
        loop {
            guard.move_forward(&obstacles);
            if guard.position.x < 0
                || guard.position.y < 0
                || guard.position.x == width as i32
                || guard.position.y == height as i32
            {
                break;
            }

            if visited.contains(&(guard.position, guard.direction)) {
                valid += 1;
                break;
            }
            visited.insert((guard.position, guard.direction));
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(&input), 41);
        assert_eq!(part2(&input), 6);
    }
}

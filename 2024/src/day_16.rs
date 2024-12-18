#![allow(unused_variables)]

struct Maze {
    walls: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

fn parse(input: &str) -> Maze {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let width = input.find('\n').unwrap() - 2;
    let height = input.len() / (width + 2) - 2;
    let mut walls = vec![vec![false; height]; width];

    let mut x = 0;
    let mut y = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                x = 0;
                y += 1;
            }
            '#' if x > 0 && y > 0 && x <= width && y <= height => {
                walls[x - 1][y - 1] = true;
            }
            'S' => start = (x - 1, y - 1),
            'E' => end = (x - 1, y - 1),
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

fn find_min_path(
    maze: &Maze,
    position: (usize, usize),
    score: u64,
    direction: usize,
    done: &mut Vec<Vec<Vec<u64>>>,
) -> u64 {
    if done.is_empty() {
        done.extend(vec![
            vec![vec![u64::MAX - 4000; maze.height]; maze.width];
            4
        ]);
    }

    if position == maze.end {
        return score;
    }

    // check if already visited with better score
    if score >= done[direction][position.0][position.1] {
        return u64::MAX;
    }
    // check if already visted rotation with better score
    for d in 1..4 {
        if score > done[(direction + d) % 4][position.0][position.1] + d as u64 % 2 * 1000 {
            return u64::MAX;
        }
    }

    done[direction][position.0][position.1] = score;

    let next_position = [
        (position.0 + 1, position.1),
        (position.0, position.1 + 1),
        (position.0.wrapping_sub(1), position.1),
        (position.0, position.1.wrapping_sub(1)),
    ];

    [
        (next_position[direction], direction, score + 1),
        (position, (direction + 1) % 4, score + 1000),
        (position, (direction + 3) % 4, score + 1000),
    ]
    .into_iter()
    .map(|(p, d, s)| {
        if p.0 >= maze.width || p.1 >= maze.height || maze.walls[position.0][position.1] {
            return u64::MAX;
        }

        find_min_path(maze, p, s, d, done)
    })
    .min()
    .unwrap()
}

pub fn part1(input: &str) -> String {
    let maze = parse(&input);
    find_min_path(&maze, maze.start, 0, 0, &mut Vec::new()).to_string()
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
        assert_eq!(part2(&EXAMPLE1), "todo");
    }
}

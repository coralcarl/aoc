#![allow(unused_variables)]

use std::collections::BTreeSet;

const SIZE: usize = if cfg!(test) { 7 } else { 71 };

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0; SIZE]; SIZE];
    for (i, line) in input.lines().enumerate() {
        let (x, y) = line.split_once(',').unwrap();
        grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = i + 1;
    }
    grid
}

fn manhatten(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[derive(Ord, PartialOrd)]
struct Node(usize, (usize, usize));

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

fn shortest_path(grid: &Vec<Vec<usize>>, nsecs: usize) -> Option<usize> {
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (SIZE - 1, SIZE - 1);
    let mut open = BTreeSet::from([Node(0, start)]);
    let mut parent: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; SIZE]; SIZE];
    let mut score = vec![vec![usize::MAX; SIZE]; SIZE];
    score[0][0] = 0;

    while let Some(Node(_, current)) = open.pop_first() {
        let (x, y) = current;
        if current == end {
            let mut steps = 0;
            let mut p = parent[current.1][current.0];
            while p.is_some() {
                let (x, y) = p.unwrap();
                steps += 1;
                p = parent[y][x];
            }
            return Some(steps);
        }

        let neighbors = [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ];

        for neighbor in neighbors {
            let (nx, ny) = neighbor;
            if nx >= SIZE || ny >= SIZE || (grid[ny][nx] > 0 && grid[ny][nx] <= nsecs) {
                continue;
            }

            let score_next = score[y][x] + 1;
            if score_next < score[ny][nx] {
                parent[ny][nx] = Some(current);
                score[ny][nx] = score_next;
                let heuristic_score = score_next + manhatten(current, neighbor);
                if !open.insert(Node(heuristic_score, neighbor)) {
                    open.replace(Node(heuristic_score, neighbor));
                }
            }
        }
    }

    None
}

fn search_cutoff(grid: &Vec<Vec<usize>>) -> usize {
    let mut first = 0;
    let mut last = grid
        .iter()
        .flatten()
        .filter(|&&n| n > 0)
        .collect::<Vec<_>>()
        .len()
        - 1;

    while first < last {
        let mid = (first + last) / 2;

        if shortest_path(grid, mid).is_some() {
            first = mid + 1;
        } else {
            last = mid;
        }
    }
    first
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(&input);
    shortest_path(&grid, 1024).unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(&input);
    let cutoff = search_cutoff(&grid);
    input.lines().skip(cutoff-1).next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        let grid = parse_grid(&EXAMPLE1);
        let result = shortest_path(&grid, 12).unwrap().to_string();
        assert_eq!(result, "22");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "6,1");
    }
}

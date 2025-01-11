use std::collections::HashSet;

use aoclib::solution::Solution;

fn parse(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);

    let mut lines = input.lines();

    let mut walls = vec![vec![false; width]; height];
    let mut guard = (0, 0);

    for (r, line) in lines.enumerate() {
        for (c, val) in line.bytes().enumerate() {
            match val {
                b'#' => walls[r][c] = true,
                b'^' => guard = (c, r),
                _ => (),
            }
        }
    }

    (walls, guard)
}

fn pos_idx(pos: (usize, usize)) -> usize {
    (pos.0 << 16) | pos.1
}

fn idx_pos(idx: usize) -> (usize, usize) {
    (idx >> 16, 0xFFFF & idx)
}

fn posdir_idx(pos: (usize, usize), dir: (isize, isize)) -> usize {
    (pos_idx(pos) << 16)
        | match dir {
            (0, -1) => 0,
            (1, 0) => 1,
            (0, 1) => 2,
            (-1, 0) => 3,
            _ => unreachable!(),
        }
}

fn patrol(mut guard: (usize, usize), walls: &Vec<Vec<bool>>) -> HashSet<usize> {
    let mut route = HashSet::from_iter([pos_idx(guard)]);
    let mut dir = (0, -1);

    loop {
        let next = (
            guard.0.wrapping_add_signed(dir.0),
            guard.1.wrapping_add_signed(dir.1),
        );

        if !(next.0 < walls[0].len() && next.1 < walls.len()) {
            return route;
        }

        if walls[next.1][next.0] {
            dir = (-dir.1, dir.0);
        } else {
            guard = next;
            route.insert(pos_idx(guard));
        }
    }
}

fn is_loop(
    mut guard: (usize, usize),
    new_obstruction: (usize, usize),
    walls: &Vec<Vec<bool>>,
) -> bool {
    let mut dir = (0, -1);
    let mut route: HashSet<usize> = HashSet::from_iter([posdir_idx(guard, dir)]);

    loop {
        let next = (
            guard.0.wrapping_add_signed(dir.0),
            guard.1.wrapping_add_signed(dir.1),
        );

        if !(next.0 < walls[0].len() && next.1 < walls.len()) {
            return false;
        }

        if walls[next.1][next.0] || next == new_obstruction {
            dir = (-dir.1, dir.0);
        } else {
            guard = next;
            if !route.insert(posdir_idx(guard, dir)) {
                return true;
            }
        }
    }
}

pub fn part1(input: &str) -> Solution {
    let (walls, guard) = parse(&input);
    Solution::Usize(patrol(guard, &walls).len())
}

pub fn part2(input: &str) -> Solution {
    assert_eq!(idx_pos(pos_idx((5, 5))), (5, 5));
    let (walls, guard) = parse(&input);
    let route = patrol(guard, &walls);
    Solution::Usize(route.into_iter().filter(|&pos| is_loop(guard, idx_pos(pos), &walls)).count())
}

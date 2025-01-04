#![allow(unused_variables)]

use std::collections::{BinaryHeap, HashMap};

type Walls = Vec<Vec<bool>>;

fn parse(input: &str) -> Walls {
    input
        .lines()
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect()
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    pos: (usize, usize),
    dir: (isize, isize),
}

fn neighbors(state: &State, walls: &Walls) -> Vec<(State, usize)> {
    [
        (
            State {
                pos: (
                    state.pos.0.wrapping_add_signed(state.dir.0),
                    state.pos.1.wrapping_add_signed(state.dir.1),
                ),
                dir: state.dir,
            },
            1,
        ),
        (
            State {
                pos: (
                    state.pos.0.wrapping_add_signed(-state.dir.1),
                    state.pos.1.wrapping_add_signed(state.dir.0),
                ),
                dir: (-state.dir.1, state.dir.0),
            },
            1001,
        ),
        (
            State {
                pos: (
                    state.pos.0.wrapping_add_signed(state.dir.1),
                    state.pos.1.wrapping_add_signed(-state.dir.0),
                ),
                dir: (state.dir.1, -state.dir.0),
            },
            1001,
        ),
    ]
    .into_iter()
    .filter_map(|(state, cost)| {
        if !walls[state.pos.1][state.pos.0] {
            Some((state, cost))
        } else {
            None
        }
    })
    .collect()
}

fn heuristic(state: &State, end: (usize, usize)) -> usize {
    if state.pos.0 != end.0 && state.pos.1 != end.1 {
        return state.pos.0.abs_diff(end.0) + state.pos.1.abs_diff(end.1) + 1000;
    } else {
        return state.pos.0.abs_diff(end.0) + state.pos.1.abs_diff(end.1);
    }
}

struct PState {
    estimated_cost: usize,
    current_cost: usize,
    state: State,
}

impl PState {
    fn new(estimated_cost: usize, current_cost: usize, state: State) -> Self {
        Self {
            estimated_cost,
            current_cost,
            state,
        }
    }
}

impl Eq for PState {}
impl PartialEq for PState {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.current_cost.eq(&other.current_cost)
    }
}

impl PartialOrd for PState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            std::cmp::Ordering::Equal => self.current_cost.cmp(&other.current_cost),
            ord => ord,
        }
    }
}

fn reverse_path(
    visited: &HashMap<State, (Option<State>, usize)>,
    current: &State,
) -> Vec<(usize, usize)> {
    let mut path = vec![current.pos];
    let mut current_state = current;
    while let Some((Some(parent), _)) = visited.get(&current_state) {
        path.push(parent.pos);
        current_state = parent;
    }
    path.reverse();
    path
}

fn cheapest_path(maze: Walls) -> Option<(Vec<(usize, usize)>, usize)> {
    let start = State {
        pos: (1, maze.len() - 2),
        dir: (1, 0),
    };
    let end = (maze[0].len() - 2, 1);

    let mut open: BinaryHeap<PState> = BinaryHeap::new();
    let mut visited: HashMap<State, (Option<State>, usize)> = HashMap::new();

    open.push(PState::new(0, 0, start));
    visited.insert(start, (None, 0));

    while let Some(PState {
        estimated_cost,
        current_cost,
        state: current,
    }) = open.pop()
    {
        if current.pos == end {
            return Some((reverse_path(&visited, &current), current_cost));
        }

        if current_cost > visited.get(&current).unwrap().1 {
            continue;
        }

        for (successor, move_cost) in neighbors(&current, &maze) {
            let new_cost = current_cost + move_cost;

            match visited.entry(successor) {
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert((Some(current), new_cost));
                }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        e.insert((Some(current), new_cost));
                    } else {
                        continue;
                    }
                }
            }

            let h_cost = heuristic(&successor, end);

            open.push(PState::new(new_cost + h_cost, new_cost, successor));
        }
    }

    None
}

pub fn part1(input: &str) -> String {
    let (path, cost) = cheapest_path(parse(&input)).unwrap();
    cost.to_string()
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

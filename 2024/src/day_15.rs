#![allow(unused_variables)]

use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Entity {
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

type Warehouse = Vec<Vec<Option<Entity>>>;

type Inputs = Vec<(isize, isize)>;

fn parse(input: &str) -> (Warehouse, Inputs, (usize, usize)) {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let width = input.find('\n').unwrap();
    let height = map.len() / (width);
    let mut robot = (0, 0);

    let mut warehouse = vec![vec![None; width]; height];

    for (y, row) in map.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    warehouse[y][x] = Some(Entity::Wall);
                }
                'O' => {
                    warehouse[y][x] = Some(Entity::Box);
                }
                '@' => {
                    warehouse[y][x] = Some(Entity::Robot);
                    robot = (x, y);
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }

    let movements = movements
        .chars()
        .filter_map(|c| match c {
            '^' => Some((0, -1)),
            'v' => Some((0, 1)),
            '>' => Some((1, 0)),
            '<' => Some((-1, 0)),
            _ => None,
        })
        .collect();

    (warehouse, movements, robot)
}

fn next_pos(pos: (usize, usize), mv_vec: (isize, isize)) -> (usize, usize) {
    (
        pos.0.wrapping_add_signed(mv_vec.0),
        pos.1.wrapping_add_signed(mv_vec.1),
    )
}

fn move_robot(robot: &mut (usize, usize), mv_vec: (isize, isize), warehouse: &mut Warehouse) {
    let mut open = BTreeSet::new();
    let mut closed = HashMap::new();
    open.insert(*robot);

    while let Some(pos) = open.pop_first() {
        let obj = warehouse[pos.1][pos.0].unwrap();
        match obj {
            Entity::Wall => {
                return;
            }
            Entity::BoxLeft | Entity::BoxRight if (mv_vec == (0, 1) || mv_vec == (0, -1)) => {
                let pos2 = match obj {
                    Entity::BoxLeft => (pos.0 + 1, pos.1),
                    Entity::BoxRight => (pos.0 - 1, pos.1),
                    _ => panic!(),
                };

                if !closed.contains_key(&pos2) {
                    open.insert(pos2);
                }
            }
            _ => (),
        }

        let new_pos = next_pos(pos, mv_vec);
        if warehouse[new_pos.1][new_pos.0].is_some() && !closed.contains_key(&new_pos) {
            open.insert(new_pos);
        }
        closed.insert(pos, new_pos);
    }

    let temp: Vec<((usize, usize), Option<Entity>)> = closed
        .into_iter()
        .map(|(old, new)| (new, warehouse[old.1][old.0].take()))
        .collect();

    for (pos, obj) in temp {
        if obj == Some(Entity::Robot) {
            *robot = pos;
        }
        warehouse[pos.1][pos.0] = obj;
    }
}

fn gps_sum(warehouse: &Warehouse) -> usize {
    let mut result = 0;
    for y in 0..warehouse.len() {
        for x in 0..warehouse[y].len() {
            match warehouse[y][x] {
                Some(Entity::Box) | Some(Entity::BoxLeft) => result += 100 * y + x,
                _ => (),
            }
        }
    }
    result
}

fn run_game_loop(warehouse: &mut Warehouse, inputs: Inputs, mut robot: (usize, usize)) {
    for mv_vec in inputs {
        move_robot(&mut robot, mv_vec, warehouse);
        // debug_warehouse(&warehouse, mv_vec);
    }
}

fn widen_map(warehouse: &mut Warehouse) {
    for row in warehouse.iter_mut() {
        let mut new_row = Vec::with_capacity(row.len() * 2);
        for obj in row.iter() {
            match obj {
                Some(Entity::Box) => {
                    new_row.push(Some(Entity::BoxLeft));
                    new_row.push(Some(Entity::BoxRight));
                }
                Some(Entity::Robot) => {
                    new_row.push(Some(Entity::Robot));
                    new_row.push(None);
                }
                o => {
                    new_row.push(*o);
                    new_row.push(*o);
                }
            }
        }
        *row = new_row;
    }
}

#[allow(unused)]
fn debug_warehouse(warehouse: &Warehouse, mv_vec: (isize, isize)) {
    let dir = match mv_vec {
        (1, 0) => '>',
        (-1, 0) => '<',
        (0, 1) => 'v',
        (0, -1) => '^',
        (0, 0) => '@',
        _ => panic!(),
    };
    for row in warehouse {
        for obj in row {
            print!(
                "{}",
                match obj {
                    None => '.',
                    Some(Entity::Wall) => '#',
                    Some(Entity::Robot) => dir,
                    Some(Entity::Box) => 'O',
                    Some(Entity::BoxLeft) => '[',
                    Some(Entity::BoxRight) => ']',
                }
            );
        }
        println!();
    }

    std::io::stdin().read_line(&mut String::new());
    println!();
}

pub fn part1(input: &str) -> String {
    let (mut warehouse, inputs, robot) = parse(&input);
    run_game_loop(&mut warehouse, inputs, robot);
    gps_sum(&warehouse).to_string()
}

pub fn part2(input: &str) -> String {
    let (mut warehouse, inputs, mut robot) = parse(&input);
    widen_map(&mut warehouse);
    robot.0 *= 2;
    run_game_loop(&mut warehouse, inputs, robot);
    gps_sum(&warehouse).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const EXAMPLE3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "2028");
        assert_eq!(part1(&EXAMPLE2), "10092");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE2), "9021");
        assert_eq!(part2(&EXAMPLE3), "618");
    }
}

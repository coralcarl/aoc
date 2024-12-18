#![allow(unused_variables)]

use std::collections::HashSet;

struct Warehouse {
    boxes: HashSet<(u64, u64)>,
    walls: HashSet<(u64, u64)>,
    robot: (u64, u64),
}

fn parse(input: &str) -> (Warehouse, String) {
    let (warehouse, movements) = input.split_once("\n\n").unwrap();

    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    let mut robot = (0, 0);

    for (y, row) in warehouse.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((x as u64, y as u64));
                }
                'O' => {
                    boxes.insert((x as u64, y as u64));
                }
                '@' => {
                    robot = (x as u64, y as u64);
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }

    (
        Warehouse {
            boxes,
            walls,
            robot,
        },
        movements.replace("\n", ""),
    )
}

fn move_robot(warehouse: &mut Warehouse, movements: &str) {
    for mv in movements.chars() {
        let mv = match mv {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => panic!(),
        };

        let new_pos = (
            warehouse.robot.0.wrapping_add_signed(mv.0),
            warehouse.robot.1.wrapping_add_signed(mv.1),
        );

        if warehouse.walls.contains(&new_pos) {
            continue;
        }

        if let Some(&mv_box) = warehouse.boxes.get(&new_pos) {
            let mut box_new_pos = (
                mv_box.0.wrapping_add_signed(mv.0),
                mv_box.1.wrapping_add_signed(mv.1),
            );
            while warehouse.boxes.contains(&box_new_pos) {
                box_new_pos = (
                    box_new_pos.0.wrapping_add_signed(mv.0),
                    box_new_pos.1.wrapping_add_signed(mv.1),
                );
            }
            if warehouse.walls.contains(&box_new_pos) {
                continue;
            }
            warehouse.boxes.remove(&mv_box);
            warehouse.boxes.insert(box_new_pos);
        }
        warehouse.robot = new_pos;
    }
}

fn gps(x: u64, y: u64) -> u64 {
    100 * y + x
}

pub fn part1(input: &str) -> String {
    let (mut warehouse, movements) = parse(&input);
    move_robot(&mut warehouse, &movements);
    warehouse
        .boxes
        .iter()
        .map(|b| gps(b.0, b.1))
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    0.to_string()
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "2028");
        assert_eq!(part1(&EXAMPLE2), "10092");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "0");
    }
}

use std::collections::HashSet;

fn deliver_presents(input: &str) -> HashSet<isize> {
    let mut pos = 0;
    let mut visited = HashSet::from_iter([pos]);

    for b in input.bytes() {
        match b {
            b'>' => pos += 1,
            b'<' => pos -= 1,
            b'v' => pos += 256,
            b'^' => pos -= 256,
            _ => (),
        }
        visited.insert(pos);
    }
    visited
}

pub fn part1(input: &str) -> String {
    deliver_presents(&input).len().to_string()
}

pub fn part2(input: &str) -> String {
    let santa_input: String = input.chars().step_by(2).collect();
    let robot_input: String = input.chars().skip(1).step_by(2).collect();
    let mut visited = deliver_presents(&santa_input);
    visited.extend(deliver_presents(&robot_input));
    visited.len().to_string()
}

#![allow(unused_variables)]

use std::collections::HashMap;

struct Keypad {
    key_to_pos: fn(u8) -> (usize, usize),
    accept: (usize, usize),
    gap: (usize, usize),
}

const NUMPAD: Keypad = Keypad {
    key_to_pos: |k| match k {
        b'7' => (0, 0),
        b'8' => (1, 0),
        b'9' => (2, 0),
        b'4' => (0, 1),
        b'5' => (1, 1),
        b'6' => (2, 1),
        b'1' => (0, 2),
        b'2' => (1, 2),
        b'3' => (2, 2),
        b'0' => (1, 3),
        b'A' => (2, 3),
        _ => unreachable!(),
    },
    accept: (2, 3),
    gap: (0, 3),
};

const DIRPAD: Keypad = Keypad {
    key_to_pos: |k| match k {
        b'^' => (1, 0),
        b'A' => (2, 0),
        b'<' => (0, 1),
        b'v' => (1, 1),
        b'>' => (2, 1),
        _ => unreachable!(),
    },
    accept: (2, 0),
    gap: (0, 0),
};

impl Keypad {
    fn extrapolate(&self, input: &str) -> String {
        let mut arm = self.accept;
        let mut result = String::new();

        for key in input.bytes().map(|b| (self.key_to_pos)(b)) {
            let mut horizontal = String::new();
            let mut vertical = String::new();

            for _ in arm.0..key.0 {
                horizontal.push('>');
            }
            for _ in key.0..arm.0 {
                horizontal.push('<');
            }
            for _ in arm.1..key.1 {
                vertical.push('v');
            }
            for _ in key.1..arm.1 {
                vertical.push('^');
            }

            if arm.1 == self.gap.1 && key.0 == 0 {
                result += &(vertical + &horizontal);
            } else if arm.0 == 0 && key.1 == self.gap.1 || arm.0 > key.0 {
                result += &(horizontal + &vertical);
            } else {
                result += &(vertical + &horizontal);
            }

            result.push('A');

            arm = key;
        }

        result
    }
}

fn complexity(input: &str, extrapolate_len: usize) -> usize {
    extrapolate_len * input[0..input.len() - 1].parse::<usize>().unwrap()
}

fn incept(input: &str, depth: usize, visited: &mut HashMap<(String, usize), usize>) -> usize {
    if depth == 0 {
        return input.len();
    }

    input
        .split_inclusive('A')
        .map(|sub| {
            if let Some(result) = visited.get(&(sub.to_string(), depth)) {
                return *result;
            }
            let extrapolated = DIRPAD.extrapolate(sub);
            let result = incept(&extrapolated, depth - 1, visited);
            visited.insert((sub.to_string(), depth), result);
            result
        })
        .sum()
}

fn keypad_inception(input: &str, depth: usize) -> usize {
    let numpad_inputs: Vec<_> = input.lines().collect();
    let extrapolated: Vec<_> = numpad_inputs
        .iter()
        .map(|output| NUMPAD.extrapolate(&output.trim_end()))
        .collect();

    let mut visited = HashMap::new();
    let mut result = 0;

    for (input, ext) in numpad_inputs.iter().zip(extrapolated) {
        result += complexity(input, incept(&ext, depth, &mut visited));
    }

    result
}

pub fn part1(input: &str) -> String {
    keypad_inception(input, 2).to_string()
}

pub fn part2(input: &str) -> String {
    keypad_inception(input, 25).to_string()
}

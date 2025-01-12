use aoclib::solution::Solution;

struct City {
    antenna_groups: Vec<Vec<(usize, usize)>>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> City {
    let mut groups = vec![vec![]; 100];

    let (mut row, mut col) = (0, 0);
    for b in input[..input.len() - 1].bytes() {
        match b {
            b'.' => (),
            b'\n' => {
                col = 0;
                row += 1;
            }
            _ => groups[(b - b'0') as usize].push((col, row)),
        }

        if b != b'\n' {
            col += 1;
        }
    }

    City {
        antenna_groups: groups.into_iter().filter(|v| !v.is_empty()).collect(),
        width: col,
        height: row,
    }
}

fn pos_to_idx(pos: (usize, usize), offset: usize) -> usize {
    pos.0 + pos.1 * offset
}

pub fn part1(input: &str) -> Solution {
    let city = parse(&input);
    let mut frequencies = vec![false; city.width * city.height];

    for antennas in city.antenna_groups.iter() {
        for (i, a1) in antennas.iter().enumerate() {
            for a2 in antennas.iter().skip(i + 1) {
                for antinode in [
                    ((2 * a2.0).wrapping_sub(a1.0), (2 * a2.1).wrapping_sub(a1.1)),
                    ((2 * a1.0).wrapping_sub(a2.0), (2 * a1.1).wrapping_sub(a2.1)),
                ] {
                    if antinode.0 < city.width && antinode.1 < city.height {
                        frequencies[pos_to_idx(antinode, city.width)] = true;
                    }
                }
            }
        }
    }

    Solution::Usize(frequencies.into_iter().filter(|&f| f).count())
}

pub fn part2(input: &str) -> Solution {
    let city = parse(&input);
    let mut frequencies = vec![false; city.width * city.height];

    for antennas in city.antenna_groups.iter() {
        for (i, a1) in antennas.iter().enumerate() {
            for a2 in antennas.iter().skip(i + 1) {
                let (dx, dy) = (a2.0.wrapping_sub(a1.0), a2.1.wrapping_sub(a1.1));
                let mut next = (a2.0.wrapping_sub(dx), a2.1.wrapping_sub(dy));
                while next.0 < city.width && next.1 < city.height {
                    frequencies[pos_to_idx(next, city.width)] = true;
                    next = (next.0.wrapping_sub(dx), next.1.wrapping_sub(dy));
                }
                let mut next = (a1.0.wrapping_add(dx), a1.1.wrapping_add(dy));
                while next.0 < city.width && next.1 < city.height {
                    frequencies[pos_to_idx(next, city.width)] = true;
                    next = (next.0.wrapping_add(dx), next.1.wrapping_add(dy));
                }
            }
        }
    }

    Solution::Usize(frequencies.into_iter().filter(|&f| f).count())
}

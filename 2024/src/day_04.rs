use aoclib::solution::Solution;

pub fn part1(input: &str) -> Solution {
    let g: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
    const SMAX: [u8; 4] = [b'S', b'A', b'M', b'X'];

    let mut occurences = 0;

    for r in 0..g.len() {
        for c in 0..g.len() - 3 {
            if g[r][c..c + 4] == XMAS || g[r][c..c + 4] == SMAX {
                occurences += 1;
            }
        }
    }

    for r in 0..g.len() - 3 {
        for c in 0..g.len() {
            let vertical = [g[r][c], g[r + 1][c], g[r + 2][c], g[r + 3][c]];
            if vertical == XMAS || vertical == SMAX {
                occurences += 1;
            }
        }
    }

    for r in 0..g.len() - 3 {
        for c in 3..g.len() {
            let diagonal = [g[r][c], g[r + 1][c - 1], g[r + 2][c - 2], g[r + 3][c - 3]];
            if diagonal == XMAS || diagonal == SMAX {
                occurences += 1;
            }
        }
    }

    for r in 0..g.len() - 3 {
        for c in 0..g.len() - 3 {
            let diagonal = [g[r][c], g[r + 1][c + 1], g[r + 2][c + 2], g[r + 3][c + 3]];
            if diagonal == XMAS || diagonal == SMAX {
                occurences += 1;
            }
        }
    }

    Solution::Usize(occurences)
}

pub fn part2(input: &str) -> Solution {
    let g: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    Solution::Usize(
        (1..g.len() - 1)
            .map(|r| {
                (1..g[0].len() - 1)
                    .filter(|&c| {
                        g[r][c] == b'A'
                            && (g[r - 1][c - 1] == b'S' && g[r + 1][c + 1] == b'M'
                                || g[r - 1][c - 1] == b'M' && g[r + 1][c + 1] == b'S')
                            && (g[r + 1][c - 1] == b'S' && g[r - 1][c + 1] == b'M'
                                || g[r + 1][c - 1] == b'M' && g[r - 1][c + 1] == b'S')
                    })
                    .count()
            })
            .sum::<usize>(),
    )
}

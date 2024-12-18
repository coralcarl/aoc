pub fn part1(input: &str) -> String {
    let lines = input
        .trim()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut horizontals = 0;
    let mut verticals = 0;
    let mut diagonals = 0;

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            if line[col] != 'X' {
                col += 1;
                continue;
            }
            // up-left
            if row > 2
                && col > 2
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row - offset][col - offset] == chr)
            {
                diagonals += 1;
            }
            // down-left
            if row + 3 < lines.len()
                && col > 2
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row + offset][col - offset] == chr)
            {
                diagonals += 1;
            }
            // up-right
            if row > 2
                && col + 3 < line.len()
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row - offset][col + offset] == chr)
            {
                diagonals += 1;
            }
            // down-right
            if row + 3 < lines.len()
                && col + 3 < line.len()
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row + offset][col + offset] == chr)
            {
                diagonals += 1;
            }
            // up
            if row > 2
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row - offset][col] == chr)
            {
                verticals += 1;
            }
            // down
            if row + 3 < lines.len()
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row + offset][col] == chr)
            {
                verticals += 1;
            }
            // left
            if col > 2
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| line[col - offset] == chr)
            {
                horizontals += 1;
            }
            // right
            if col + 3 < line.len()
                && "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| line[col + offset] == chr)
            {
                horizontals += 1;
                col += 3
            }

            col += 1;
        }
    }

    (horizontals + verticals + diagonals).to_string()
}

pub fn part2(input: &str) -> String {
    let lines = input
        .trim()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut x = 0;

    for (row, line) in lines[..lines.len() - 1].iter().enumerate().skip(1) {
        for (col, c) in line[..line.len() - 1].iter().enumerate().skip(1) {
            if line[col] != 'A' {
                continue;
            }

            if *c == 'A'
                && ("MAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row + offset - 1][col + offset - 1] == chr)
                    || "MAS"
                        .chars()
                        .enumerate()
                        .all(|(offset, chr)| lines[row + 1 - offset][col + 1 - offset] == chr))
                && ("MAS"
                    .chars()
                    .enumerate()
                    .all(|(offset, chr)| lines[row + 1 - offset][col + offset - 1] == chr)
                    || "MAS"
                        .chars()
                        .enumerate()
                        .all(|(offset, chr)| lines[row - 1 + offset][col + 1 - offset] == chr))
            {
                x += 1;
            }
        }
    }
    x.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(&input), "18");
        assert_eq!(part2(&input), "9");
    }
}

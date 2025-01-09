pub fn part1(input: &str) -> String {
    input
        .split(['m', ')'])
        .map(|token| {
            if token.starts_with("ul(") {
                if let Some((a, b)) = token[3..].split_once(',') {
                    return a.parse::<usize>().unwrap_or_default()
                        * b.parse::<usize>().unwrap_or_default();
                }
            }
            0
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut enabled = true;
    input
        .split(['m', 'd', ')'])
        .map(|token| {
            if token == "on't(" {
                enabled = false;
            } else if token == "o(" {
                enabled = true;
            } else if enabled && token.starts_with("ul(") {
                if let Some((a, b)) = token[3..].split_once(',') {
                    return a.parse::<usize>().unwrap_or_default()
                        * b.parse::<usize>().unwrap_or_default();
                }
            } else if token.starts_with("ul(") {
            }
            0
        })
        .sum::<usize>()
        .to_string()
}

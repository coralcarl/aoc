pub fn part1(input: &str) -> String {
    input
        .bytes()
        .map(|b| match b {
            b'(' => 1,
            b')' => -1,
            _ => 0,
        })
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut floor = 0;
    for (i, b) in input.bytes().enumerate() {
        match b {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return (i+1).to_string();
        }
    }
    "none".to_string()
}

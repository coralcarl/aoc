pub fn part1(input: &str) -> String {
    let input = input.trim_end();
    let mut i = 1;
    loop {
        let s = format!("{}{i}", input);
        let digest = md5::compute(s.as_bytes());
        let hash = digest.as_slice();
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            break;
        }
        i += 1;
    }
    i.to_string()
}

pub fn part2(input: &str) -> String {
    let input = input.trim_end();
    let mut i = 1;
    loop {
        let s = format!("{}{i}", input);
        let digest = md5::compute(s.as_bytes());
        let hash = digest.as_slice();
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            break;
        }
        i += 1;
    }
    i.to_string()
}

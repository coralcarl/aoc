pub fn part1(input: &str) -> String {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut input = input.bytes();

    while let Some(b) = input.next() {
        let values = match b {
            b'#' => {
                locks.push(vec![0u8; 5]);
                locks.last_mut()
            }
            b'.' => {
                keys.push(vec![0u8; 5]);
                keys.last_mut()
            }
            _ => unreachable!(),
        }
        .unwrap();

        while let Some(b) = input.next() {
            if b == b'\n' {
                break;
            }
        }

        for _ in 0..5 {
            for i in 0..6 {
                match input.next() {
                    Some(b'#') => values[i] += 1,
                    _ => (),
                }
            }
        }

        while let Some(b) = input.next() {
            if b == b'\n' {
                break;
            }
        }

        input.next();
    }

    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| lock.iter().zip(key.iter()).all(|(l, k)| *l + *k < 6))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(_: &str) -> String {
    "free".to_string()
}

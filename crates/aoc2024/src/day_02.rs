fn bad_level(levels: &Vec<i16>) -> Option<usize> {
    let inc = levels[1] > levels[0];

    for i in 0..levels.len() - 1 {
        let diff = levels[i + 1] - levels[i];
        if diff > 3 || diff < -3 || diff == 0 || (inc && diff < 0) || (!inc && diff > 0) {
            return Some(i);
        }
    }

    None
}

pub fn part1(input: &str) -> u64 {
    let mut safe = 0;

    for row in input.trim().split('\n') {
        let numbers = row
            .split(' ')
            .map(|x| x.parse::<i16>().expect(&format!("{x} not valid")))
            .collect();
        if bad_level(&numbers).is_none() {
            safe += 1;
        }
    }
    safe
}

pub fn part2(input: &str) -> u64 {
    let mut safe = 0;

    for row in input.trim().split('\n') {
        let numbers = row
            .split(' ')
            .map(|x| x.parse::<i16>().expect(&format!("{x} not valid")))
            .collect();
        match bad_level(&numbers) {
            Some(i) if i == 1 => {
                if bad_level(&numbers[1..].to_vec()).is_none()
                    || bad_level(&[numbers[0..1].to_vec(), numbers[2..].to_vec()].concat())
                        .is_none()
                {
                    safe += 1;
                }
            }
            Some(i) => {
                if bad_level(&[numbers[..i].to_vec(), numbers[i + 1..].to_vec()].concat()).is_none()
                    || bad_level(&[numbers[..i + 1].to_vec(), numbers[i + 2..].to_vec()].concat())
                        .is_none()
                {
                    safe += 1;
                }
            }
            None => safe += 1,
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}

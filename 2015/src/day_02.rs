pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split('x')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let sides = [nums[0] * nums[1], nums[0] * nums[2], nums[1] * nums[2]];
            sides.iter().sum::<usize>() * 2 + sides.iter().min().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split('x')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            nums.sort();
            (nums[0]+ nums[1] ) * 2 + nums.iter().product::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

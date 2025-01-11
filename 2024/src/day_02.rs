use aoclib::solution::Solution;

fn find_bad_level(report: &[usize]) -> Option<usize> {
    let inc = report[0] < report[1];
    for (i, l) in report.windows(2).enumerate() {
        if l[0] == l[1] || inc && l[0] > l[1] || !inc && l[0] < l[1] || l[0].abs_diff(l[1]) > 3 {
            return Some(i);
        }
    }
    None
}

fn parse_reports(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| unsafe { num.parse::<usize>().unwrap_unchecked() })
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> Solution {
    Solution::Usize(
        parse_reports(input)
            .iter()
            .filter(|report| find_bad_level(report).is_none())
            .count(),
    )
}

pub fn part2(input: &str) -> Solution {
    Solution::Usize(
        parse_reports(input)
            .iter()
            .filter(|report| match find_bad_level(report) {
                None => true,
                Some(i) if i == 1 => {
                    find_bad_level(&report[1..]).is_none()
                        || find_bad_level(&[&report[..1], &report[2..]].concat()).is_none()
                }
                Some(i) => {
                    find_bad_level(&[&report[..i], &report[i + 1..]].concat()).is_none()
                        || find_bad_level(&[&report[..i + 1], &report[i + 2..]].concat()).is_none()
                }
            })
            .count(),
    )
}

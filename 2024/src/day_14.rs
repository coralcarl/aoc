#![allow(unused_variables)]

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    xv: i64,
    yv: i64,
}

fn parse(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in input.lines() {
        let (p, v) = line[2..].split_once(" v=").unwrap();
        let (x, y) = p.split_once(',').unwrap();
        let (xv, yv) = v.split_once(',').unwrap();

        robots.push(Robot {
            x: x.parse::<i64>().unwrap(),
            y: y.parse::<i64>().unwrap(),
            xv: xv.parse::<i64>().unwrap(),
            yv: yv.parse::<i64>().unwrap(),
        });
    }
    robots
}

fn move_robots(robots: &mut Vec<Robot>, seconds: i64, width: usize, height: usize) {
    for robot in robots.iter_mut() {
        robot.x = (robot.x + robot.xv * seconds).rem_euclid(width as i64);
        robot.y = (robot.y + robot.yv * seconds).rem_euclid(height as i64);
    }
}

fn quadrant_spread(robots: &Vec<Robot>, width: usize, height: usize) -> [[usize; 2]; 2] {
    let mut quadrants = [[0, 0], [0, 0]];

    for robot in robots {
        let quad_width = width / 2;
        let quad_height = height / 2;
        if robot.x as usize == quad_width || robot.y as usize == quad_height {
            continue;
        }

        quadrants[robot.y as usize / (quad_height + 1)][robot.x as usize / (quad_width + 1)] += 1;
    }

    quadrants
}

fn variance(values: &[i64]) -> f64 {
    let mean = values.iter().map(|v| *v as f64).sum::<f64>() / values.len() as f64;
    values
        .iter()
        .map(|v| (*v as f64 - mean) * (*v as f64 - mean))
        .sum::<f64>()
        / values.len() as f64
}

fn mod_inverse(a: i64, n: i64) -> Option<i64> {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = n;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        (r, new_r) = (new_r, r - quotient * new_r);
        (t, new_t) = (new_t, t - quotient * new_t);
    }

    if r > 1 {
        return None;
    }

    if t < 0 {
        t += n;
    }

    Some(t)
}

fn chinese_remainder(a: &[i64], n: &[i64]) -> Option<i64> {
    let big_n: i64 = n.iter().product();

    let mut result = 0;

    for (a_i, n_i) in a.iter().zip(n) {
        let n_i_part = big_n / n_i;
        if let Some(m_i) = mod_inverse(n_i_part, *n_i) {
            result = (result + a_i * m_i * n_i_part) % big_n;
        } else {
            return None;
        }
    }

    Some((result + big_n) % big_n)
}

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

pub fn part1(input: &str) -> String {
    let mut robots = parse(&input);
    move_robots(&mut robots, 100, WIDTH, HEIGHT);
    quadrant_spread(&robots, WIDTH, HEIGHT)
        .as_flattened()
        .iter()
        .product::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut robots = parse(&input);

    let mut x_variance = Vec::new();
    let mut y_variance = Vec::new();

    for i in 0..HEIGHT {
        move_robots(&mut robots, 1, WIDTH, HEIGHT);
        x_variance.push(variance(
            &robots.iter().map(|robot| robot.x).collect::<Vec<i64>>(),
        ));
        y_variance.push(variance(
            &robots.iter().map(|robot| robot.y).collect::<Vec<i64>>(),
        ));
    }

    let x_offset = x_variance
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index + 1)
        .unwrap();
    let y_offset = y_variance
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index + 1)
        .unwrap();

    chinese_remainder(&[x_offset as i64, y_offset as i64], &[101, 103])
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::*;

    const EXAMPLE1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let mut robots = parse(&EXAMPLE1);
        move_robots(&mut robots, 100, 11, 7);
        assert_eq!(
            quadrant_spread(&robots, 11, 7)
                .as_flattened()
                .iter()
                .product::<usize>(),
            12
        );
    }
}

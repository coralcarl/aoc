use std::fs::read_to_string;

use criterion::{black_box, Criterion};

#[path = "../src/day_01.rs"]
mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    let days: Vec<(fn(&str) -> String, fn(&str) -> String)> = vec![
        (day_01::part1, day_01::part2),
        (day_02::part1, day_02::part2),
        (day_03::part1, day_03::part2),
        (day_04::part1, day_04::part2),
    ];

    for (day, (part1, part2)) in days.iter().enumerate() {
        let path = format!("input/{:02}.txt", day + 1);
        let file = read_to_string(&path);

        println!("====== Day {:02} ======", day + 1);
        match file {
            Ok(input) => {
                let solution1 = part1(&input);
                let solution2 = part2(&input);
                println!("Part 1: {solution1}");
                println!("Part 2: {solution2}");
            }
            Err(_) => {
                println!("File not found");
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let days: Vec<(fn(&str) -> String, fn(&str) -> String)> = vec![
        (day_01::part1, day_01::part2),
        (day_02::part1, day_02::part2),
        (day_03::part1, day_03::part2),
        (day_04::part1, day_04::part2),
    ];

    for (day, (part1, part2)) in days.iter().enumerate() {
        let path = format!("input/{:02}.txt", day + 1);
        let file = read_to_string(&path);

        match file {
            Ok(input) => {
                c.bench_function(format!("day_{day:02} part1").as_str(), |b| {
                    b.iter(|| part1(black_box(&input)))
                });
                c.bench_function(format!("day_{day:02} part2").as_str(), |b| {
                    b.iter(|| part2(black_box(&input)))
                });
            }
            Err(_) => {
                println!("File not found");
            }
        }
    }
}

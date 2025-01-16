#![allow(dead_code, unused)]
use aoclib::{solution::run_solutions, solution::Solution};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;


fn main() {
    let solutions: Vec<(usize, (fn(&str) -> Solution, fn(&str) -> Solution))> = vec![
        (1, (day_01::part1, day_01::part2)),
        (2, (day_02::part1, day_02::part2)),
        (3, (day_03::part1, day_03::part2)),
        (4, (day_04::part1, day_04::part2)),
        (5, (day_05::part1, day_05::part2)),
        (6, (day_06::part1, day_06::part2)),
        (7, (day_07::part1, day_07::part2)),
        (8, (day_08::part1, day_08::part2)),
        (9, (day_09::part1, day_09::part2)),
        // (10, (day_10::part1, day_10::part2)),
        // (11, (day_11::part1, day_11::part2)),
        // (12, (day_12::part1, day_12::part2)),
        // (13, (day_13::part1, day_13::part2)),
        // (14, (day_14::part1, day_14::part2)),
        // (15, (day_15::part1, day_15::part2)),
        // (16, (day_16::part1, day_16::part2)),
        // (17, (day_17::part1, day_17::part2)),
        // (18, (day_18::part1, day_18::part2)),
        // (19, (day_19::part1, day_19::part2)),
        // (20, (day_20::part1, day_20::part2)),
        // (21, (day_21::part1, day_21::part2)),
        // (22, (day_22::part1, day_22::part2)),
        // (23, (day_23::part1, day_23::part2)),
        // (24, (day_24::part1, day_24::part2)),
        // (25, (day_25::part1, day_25::part2)),
    ];

    run_solutions(2024, &solutions);
}

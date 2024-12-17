use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

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

fn main() {
    let days: Vec<(fn(&str) -> u64, fn(&str) -> u64)> = vec![
        (day_01::part1, day_01::part2),
        (day_02::part1, day_02::part2),
        (day_03::part1, day_03::part2),
        (day_04::part1, day_04::part2),
        (day_05::part1, day_05::part2),
        (day_06::part1, day_06::part2),
        (day_07::part1, day_07::part2),
        (day_08::part1, day_08::part2),
        (day_09::part1, day_09::part2),
        (day_10::part1, day_10::part2),
        (day_11::part1, day_11::part2),
        (day_12::part1, day_12::part2),
        (day_13::part1, day_13::part2),
        (day_14::part1, day_14::part2),
        (day_15::part1, day_15::part2),
        (day_16::part1, day_16::part2),
    ];

    for (day, (part1, part2)) in days.iter().enumerate() {
        let input_folder: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input"].iter().collect();
        let input = aoclib::read_input(input_folder, 2024, day + 1);

        println!("========= Day {:02} =========", day + 1);
        let mut start = Instant::now();
        let solution1 = part1(&input);
        let time1 = start.elapsed();
        start = Instant::now();
        let solution2 = part2(&input);
        let time2 = start.elapsed();
        println!("{} Part 1: {solution1}", readable_duration(time1));
        println!("{} Part 2: {solution2}", readable_duration(time2));
    }
}

fn readable_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        return format!("{:6.2}s ", d.as_secs_f64());
    } else if d.as_millis() > 0 {
        return format!("{:3}ms", d.as_millis());
    } else {
        return format!("{:3}μs", d.as_micros());
    }
}

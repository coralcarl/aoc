use std::fs::read_to_string;

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

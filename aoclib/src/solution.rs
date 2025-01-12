use std::{
    fmt::Display,
    time::{Duration, Instant},
};

pub enum Solution {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    String(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Not implemented"),
            Self::String(s) => write!(f, "{s}"),
            Self::Usize(n) => write!(f, "{n}"),
            Self::U8(n) => write!(f, "{n}"),
            Self::U16(n) => write!(f, "{n}"),
            Self::U32(n) => write!(f, "{n}"),
            Self::U64(n) => write!(f, "{n}"),
        }
    }
}

type Aocfunc = fn(&str) -> Solution;

pub fn run_solutions(year: usize, solutions: &[(usize, (Aocfunc, Aocfunc))]) {
    for (day, (part1, part2)) in select_solutions(solutions) {
        let input = super::read_input(year, day);

        println!("========= Day {:02} =========", day);
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

fn select_solutions(solutions: &[(usize, (Aocfunc, Aocfunc))]) -> Vec<(usize, (Aocfunc, Aocfunc))> {
    match std::env::args().skip(1).next() {
        Some(s) if s.as_str() == "all" => solutions.to_vec(),
        Some(s) => {
            if let Ok(num) = s.parse::<usize>() {
                solutions.to_vec().into_iter().filter(|&solution| solution.0 == num).collect()
            } else {
                panic!("bad arguments");
            }
        }
        None => vec![*solutions.last().unwrap()],
    }
}

fn readable_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        return format!("{:3.1}s", d.as_secs_f64());
    } else if d.as_millis() > 0 {
        return format!("{:3}ms", d.as_millis());
    } else {
        return format!("{:3}μs", d.as_micros());
    }
}

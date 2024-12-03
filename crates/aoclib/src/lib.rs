use std::{fmt::Debug, str::FromStr};

pub fn read_number_grid<T: FromStr>(input: &str, token: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(
            line.split(token)
                .map(|x| x.parse::<T>().expect("Unable to parse"))
                .collect(),
        );
    }
    numbers
}


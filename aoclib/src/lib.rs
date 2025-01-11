use std::path::{Path, PathBuf};
use std::{fmt::Debug, str::FromStr};

use std::fs::File;
use std::io::prelude::*;

use reqwest::blocking::Client;

pub mod geometry;
pub mod solution;

static mut TOKEN: Option<&str> = None;

pub fn read_input(year: usize, day: usize) -> String {
    let input_folder: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input"].iter().collect();
    let file_path = input_folder.join(format!("{day:02}.txt"));
    let input = std::fs::read_to_string(&file_path);

    match input {
        Ok(raw) => raw,
        Err(_) => {
            let raw = download_input(year, day);
            std::fs::create_dir_all(input_folder)
                .unwrap_or_else(|_| panic!("Could not create directory for {year}"));
            let mut file = File::create(&file_path)
                .unwrap_or_else(|_| panic!("Could not create File {year}-{day:02}"));
            file.write_all(&raw.bytes().collect::<Vec<u8>>())
                .unwrap_or_else(|_| panic!("Could not write data to file {year}-{day:02}"));
            raw
        }
    }
}

fn download_input(year: usize, day: usize) -> String {
    let token = unsafe {
        match TOKEN {
            Some(token) => token,
            None => {
                let token_path = workspace_dir().join("token");
                &std::fs::read_to_string(token_path).unwrap_or_else(|_| panic!("No token found."))
            }
        }
    };

    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let mut response = client
        .get(url)
        .header("Cookie", format!("session={}", token.trim()))
        .send()
        .expect("No response from adventofcode.")
        .text()
        .unwrap_or_else(|_| panic!("Could not download input {year}-{day:02}"));
    if response.starts_with("Puzzle inputs differ by user.") {
        panic!("Session token invalid.");
    }
    if response.starts_with("Please don't repeatedly request this endpoint before it unlocks!") {
        panic!("Input not yet available.");
    }
    if !response.ends_with('\n') {
        response.push('\n');
    }
    response
}

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    Path::new(std::str::from_utf8(&output).unwrap().trim())
        .parent()
        .unwrap()
        .to_path_buf()
}

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

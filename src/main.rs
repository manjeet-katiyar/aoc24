use std::env;
#[allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args
        .get(1)
        .map(|arg| arg.to_string())
        .unwrap_or("./in".to_string());
    let input = read_line(&file_name);

    day6::solve(input);
}

fn read_line(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("File not found {file_path}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect()
}

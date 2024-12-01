use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = read_line("./in");

    let (a, b): (Vec<i32>, Vec<i32>) = input
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter(|&w| !w.is_empty())
                .map(|w| i32::from_str_radix(w, 10).unwrap())
                .collect::<Vec<i32>>()
        })
        .fold((Vec::new(), Vec::new()), |(mut a, mut b), parsed| {
            a.push(parsed[0]);
            b.push(parsed[1]);
            (a, b)
        });

    let mut bmp: HashMap<i32, i32> = HashMap::new();
    for &num in &b {
        let counter = bmp.entry(num).or_insert(0);
        *counter += 1;
    }

    let res: i32 = a
        .iter()
        .zip(b.iter())
        .map(|(&a_val, &b_val)| (a_val - b_val).abs())
        .sum();

    let res1: i32 = a.iter().map(|&num| num * bmp.get(&num).unwrap_or(&0)).sum();

    println!("{:?}, {:?}", res, res1);
}

fn read_line(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("File not found {file_path}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect()
}

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_map(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Unable to read line").chars().collect())
        .collect()
}

pub fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

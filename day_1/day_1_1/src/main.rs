use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sum: i32 = BufReader::new(File::open("../input.txt").unwrap()).lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .sum();

    println!("{}", sum);
}

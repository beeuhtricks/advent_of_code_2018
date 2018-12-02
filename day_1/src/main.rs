use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sum = BufReader::new(File::open("input.txt").unwrap()).lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

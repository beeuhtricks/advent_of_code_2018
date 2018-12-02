use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let diffs: Vec<i32> = BufReader::new(File::open("../input.txt").unwrap()).lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut uniq = HashSet::new();
    let mut total = 0;
    uniq.insert(total);

    'outer: loop {
        for diff in &diffs {
            total += diff;
            if uniq.contains(&total) {
                println!("{}", total);
                break 'outer;
            } else {
                uniq.insert(total);
            }
        }
    }
}

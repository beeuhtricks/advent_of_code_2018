use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let boxes: Vec<String> = BufReader::new(File::open("../input.txt").unwrap())
        .lines()
        .map(|s| s.unwrap())
        .collect();

    let twos = boxes.clone().iter().filter(|s| has_two(hash(s))).count();
    let threes = boxes.clone().iter().filter(|s| has_three(hash(s))).count();

    println!("{}", twos * threes);
}

fn hash(box_label: &str) -> HashMap<char, i32> {
    let mut freqs = HashMap::new();

    for c in box_label.chars() {
        if freqs.contains_key(&c) {
            *freqs.get_mut(&c).unwrap() += 1;
        } else {
            freqs.insert(c, 1);
        }
    }

    freqs
}

fn has_two(hashed_box: HashMap<char, i32>) -> bool {
    hashed_box.iter().any(|(_, &v)| v == 2)
}

fn has_three(hashed_box: HashMap<char, i32>) -> bool {
    hashed_box.iter().any(|(_, &v)| v == 3)
}

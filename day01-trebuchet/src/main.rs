use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const DIGITS: &[&str] = &[
    "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
    "seven", "8", "eight", "9", "nine",
];

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    };

    let sum: usize = contents
        .lines()
        .map(|line| {
            let (a, b) = DIGITS
                .iter()
                .enumerate()
                .flat_map(|(i, &n)| line.match_indices(n).map(move |(idx, _)| (idx, i / 2)))
                .minmax()
                .into_option()
                .unwrap();
            a.1 * 10 + b.1
        })
        .sum();
    println!("sum: {}", sum);
}

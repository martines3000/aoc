extern crate aoc2022;
use rustc_hash::FxHashSet;

use crate::aoc2022::utils::scanner::Scanner;

const WINDOW_SIZE: usize = 14;

fn main() {
    let input = std::fs::read_to_string("../data/in/6.in").unwrap();
    let chars = input.chars().collect::<Vec<_>>();
    let res = chars
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_, chars)| chars.iter().collect::<FxHashSet<_>>().len() == WINDOW_SIZE)
        .unwrap();

    println!("Index: {}", res.0 + WINDOW_SIZE);
}

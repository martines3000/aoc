extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;
use rustc_hash::FxHashMap;

fn main() {
    let input = std::fs::read_to_string("../data/in/3.in").unwrap();

    // HashMap of a-z and A-Z
    let mut priority_map = FxHashMap::default();
    for (idx, c) in ('a'..='z').chain('A'..='Z').enumerate() {
        priority_map.insert(c, idx + 1);
    }

    let mut count_map = FxHashMap::default();
    let mut res = 0;

    for line in input.lines() {
        // Split line in half
        let (left, right) = line.split_at(line.len() / 2);

        // Count the number of each character in the left half
        for c in left.chars() {
            *count_map.entry(c).or_insert(0) += 1;
        }

        // Find the one which is in the left half
        for c in right.chars() {
            if count_map.contains_key(&c) {
                res += priority_map[&c];
                break;
            }
        }

        count_map.clear();
    }

    println!("Total priority: {}", res);
}

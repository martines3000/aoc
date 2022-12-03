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

    let mut count_map_1 = FxHashMap::default();
    let mut count_map_2 = FxHashMap::default();
    let mut res = 0;

    // Iterate 3 lines at a time
    input
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .for_each(|chunk| {
            // Count first
            for c in chunk[0].chars() {
                *count_map_1.entry(c).or_insert(0) += 1;
            }

            // Count second
            for c in chunk[1].chars() {
                *count_map_2.entry(c).or_insert(0) += 1;
            }

            // Find in third the one that is also in first and second
            for c in chunk[2].chars() {
                if count_map_1.contains_key(&c) && count_map_2.contains_key(&c) {
                    res += priority_map[&c];
                    break;
                }
            }

            count_map_1.clear();
            count_map_2.clear();
        });
    println!("Total priority: {}", res);
}

extern crate aoc2022;
use rustc_hash::FxHashMap;

use crate::aoc2022::utils::scanner::Scanner;
use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("../data/in/6.in").unwrap();

    let mut queue = VecDeque::new();
    let mut counter = FxHashMap::default();
    let mut res = 0;

    queue.reserve(14);
    counter.reserve(14);

    for (idx, char) in input.chars().enumerate() {
        if queue.len() < 14 {
            queue.push_back(char);
            *counter.entry(char).or_insert(0) += 1;
            continue;
        }

        let val_ref = counter.get_mut(queue.front().unwrap()).unwrap();
        *val_ref -= 1;

        if *val_ref == 0 {
            counter.remove(queue.front().unwrap());
        }

        *counter.entry(char).or_insert(0) += 1;

        queue.pop_front();
        queue.push_back(char);

        if counter.len() == 14 {
            res = idx + 1;
            break;
        }
    }

    println!("Last 4: {}", queue.iter().collect::<String>());
    println!("Index: {}", res);
}

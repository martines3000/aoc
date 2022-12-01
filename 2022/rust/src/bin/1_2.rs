extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;
use std::collections::BinaryHeap;

fn main() {
    let input = std::fs::read_to_string("../data/in/1.in").unwrap();

    let mut sum = 0;
    let mut sums = BinaryHeap::new();

    for line in input.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    println!(
        "Top 3 sum: {}",
        (0..3).map(|_| sums.pop().unwrap()).sum::<i32>()
    );
}

extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;

fn main() {
    let input = std::fs::read_to_string("../data/in/1.in").unwrap();

    let mut sum = 0;
    let mut best = 0;

    for line in input.lines() {
        if line.is_empty() {
            best = best.max(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    println!("Best: {}", best);
}

extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;

fn main() {
    let input = std::fs::read_to_string("../data/in/4.in").unwrap();

    let mut counter = 0;
    let mut curr_range = Vec::new();

    for line in input.lines() {
        let ranges = line
            .split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|s| {
                curr_range = s.split('-').collect::<Vec<_>>();
                (
                    curr_range[0].parse::<i32>().unwrap(),
                    curr_range[1].parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        if (ranges[0].0 >= ranges[1].0 && ranges[0].0 <= ranges[1].1)
            || (ranges[0].1 >= ranges[1].0 && ranges[0].1 <= ranges[1].1)
            || (ranges[1].0 >= ranges[0].0 && ranges[1].0 <= ranges[0].1)
            || (ranges[1].1 >= ranges[0].0 && ranges[1].1 <= ranges[0].1)
        {
            counter += 1;
        }
    }

    println!("Fully contained: {}", counter);
}

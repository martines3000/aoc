extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;

fn main() {
    let input = std::fs::read_to_string("../data/in/2.in").unwrap();

    let mut sum = 0;
    let mut round;

    for line in input.lines() {
        round = line.split_whitespace().collect::<Vec<&str>>();

        sum += match (round[0], round[1]) {
            ("A", "X") => 4, // 3 + 1
            ("A", "Y") => 8, // 6 + 2
            ("A", "Z") => 3, // 0 + 3
            ("B", "X") => 1, // 0 + 1
            ("B", "Y") => 5, // 3 + 2
            ("B", "Z") => 9, // 6 + 3
            ("C", "X") => 7, // 6 + 1
            ("C", "Y") => 2, // 0 + 2
            ("C", "Z") => 6, // 3 + 3
            (_, _) => 0,
        };
    }

    println!("{}", sum);
}

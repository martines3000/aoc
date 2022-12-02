extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;

fn main() {
    let input = std::fs::read_to_string("../data/in/2.in").unwrap();

    let mut sum = 0;
    let mut round;

    for line in input.lines() {
        round = line.split_whitespace().collect::<Vec<&str>>();

        sum += match (round[0], round[1]) {
            ("A", "X") => 3, // 3 + 1
            ("A", "Y") => 4, // 6 + 2
            ("A", "Z") => 8, // 0 + 3
            ("B", "X") => 1, // 0 + 1
            ("B", "Y") => 5, // 3 + 2
            ("B", "Z") => 9, // 6 + 3
            ("C", "X") => 2, // 6 + 1
            ("C", "Y") => 6, // 0 + 2
            ("C", "Z") => 7, // 3 + 3
            (_, _) => 0,
        };
    }

    println!("{}", sum);
}

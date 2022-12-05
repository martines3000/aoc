extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;

fn main() {
    let input = std::fs::read_to_string("../data/in/5.in").unwrap();

    // 2-d vector of chars
    let mut cols: Vec<Vec<char>> = Vec::new();
    let mut state = 0;

    // Regex
    let digits = regex::Regex::new(r"\d+").unwrap();

    let mut curr_vals;
    let mut tmp_char;
    let mut tmp_vec = Vec::new();

    for line in input.lines() {
        if state == 0 {
            for _ in 0..(line.len() + 1) / 4 {
                cols.push(Vec::new());
            }
            state = 1;
        }

        match state {
            1 => {
                if line[1..2].eq("1") {
                    state = 2;

                    for col in cols.iter_mut() {
                        col.reverse();
                    }
                    continue;
                }

                for i in (1..line.len()).step_by(4) {
                    tmp_char = line[i..i + 1].chars().next().unwrap();
                    if tmp_char != ' ' {
                        cols[(i - 1) / 4 as usize].push(tmp_char);
                    }
                }
            }
            2 => {
                curr_vals = digits
                    .find_iter(line)
                    .map(|digit| digit.as_str().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                if curr_vals.len() == 0 {
                    continue;
                }

                tmp_vec = (0..curr_vals[0])
                    .map(|_| cols[(curr_vals[1] - 1) as usize].pop().unwrap())
                    .collect::<Vec<_>>();

                tmp_vec.reverse();

                cols[(curr_vals[2] - 1) as usize].append(&mut tmp_vec);
            }
            _ => {}
        }
    }

    println!(
        "{}",
        cols.iter_mut()
            .map(|col| col.pop().unwrap())
            .collect::<String>()
    );
}

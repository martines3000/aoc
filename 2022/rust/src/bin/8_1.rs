extern crate aoc2022;

use std::vec;

use crate::aoc2022::utils::scanner::Scanner;

fn check_visibility(trees: &Vec<Vec<u8>>, visible: &mut Vec<Vec<bool>>) {
    let mut max_visible: (u8, u8);
    for i in 0..trees.len() {
        max_visible = (0, 0);
        for j in 0..trees[0].len() {
            if i == 0 || j == 0 || i == trees.len() - 1 || j == trees[0].len() - 1 {
                visible[i][j] = true;
            } else {
                if trees[i][j] > max_visible.0 {
                    visible[i][j] = true;
                }

                if trees[i][trees[0].len() - j - 1] > max_visible.1 {
                    visible[i][trees[0].len() - j - 1] = true;
                }
            }

            max_visible = (
                max_visible.0.max(trees[i][j]),
                max_visible.1.max(trees[i][trees[0].len() - j - 1]),
            );
        }
    }

    for j in 0..trees[0].len() {
        max_visible = (0, 0);
        for i in 0..trees.len() {
            if i == 0 || j == 0 || i == trees.len() - 1 || j == trees[0].len() - 1 {
                visible[i][j] = true;
            } else {
                if trees[i][j] > max_visible.0 {
                    visible[i][j] = true;
                }
                if trees[trees.len() - i - 1][j] > max_visible.1 {
                    visible[trees.len() - i - 1][j] = true;
                }
            }

            max_visible = (
                max_visible.0.max(trees[i][j]),
                max_visible.1.max(trees[trees.len() - i - 1][j]),
            );
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../data/in/8.in").unwrap();

    let mut trees: Vec<Vec<_>> = Vec::new();
    trees.reserve(input.lines().count());

    input.lines().enumerate().for_each(|(idx, line)| {
        trees.push(Vec::new());
        trees[idx].append(
            &mut line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );
    });

    let mut visible = vec![vec![false; trees[0].len()]; trees.len()];
    check_visibility(&trees, &mut visible);

    // Count and display visible trees
    let mut count = 0;
    trees.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _)| {
            if visible[i][j] {
                count += 1;
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });

    println!("Visible trees: {}", count);
}

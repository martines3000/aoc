extern crate aoc2022;

use std::vec;

use crate::aoc2022::utils::scanner::Scanner;

fn count_view_distance(
    trees: &Vec<Vec<u8>>,
    view_distance: &mut Vec<Vec<(usize, usize, usize, usize)>>,
) {
    for i in 1..(trees.len() - 1) {
        for j in 1..(trees[0].len() - 1) {
            // Check all 4 directions from current tree and calculate the distance
            // to the nearest tree higher or equal height in each direction.
            view_distance[i][j] = (
                (0..i)
                    .rev()
                    .find_map(|k| {
                        if trees[k][j] >= trees[i][j] {
                            return Some(i - k);
                        }

                        None
                    })
                    .unwrap_or(i),
                (0..j)
                    .rev()
                    .find_map(|k| {
                        if trees[i][k] >= trees[i][j] {
                            return Some(j - k);
                        }

                        None
                    })
                    .unwrap_or(j),
                (i + 1..trees.len())
                    .find_map(|k| {
                        if trees[k][j] >= trees[i][j] {
                            return Some(k - i);
                        }
                        None
                    })
                    .unwrap_or(trees.len() - i - 1),
                (j + 1..trees[0].len())
                    .find_map(|k| {
                        if trees[i][k] >= trees[i][j] {
                            return Some(k - j);
                        }

                        None
                    })
                    .unwrap_or(trees[0].len() - j - 1),
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

    let mut view_distance = vec![vec![(0, 0, 0, 0); trees[0].len()]; trees.len()];
    count_view_distance(&trees, &mut view_distance);

    let mut max_view_distance = 0;
    view_distance.iter().for_each(|row| {
        row.iter().for_each(|dists| {
            max_view_distance = max_view_distance.max(dists.0 * dists.1 * dists.2 * dists.3);
        })
    });
    println!("Max view distance: {}", max_view_distance);
}

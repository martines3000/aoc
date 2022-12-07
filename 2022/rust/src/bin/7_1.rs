extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;
use petgraph::{visit::Dfs, Graph};
use rustc_hash::FxHashMap;

fn main() {
    let input = std::fs::read_to_string("../data/in/7.in").unwrap();

    let mut graph = Graph::<usize, usize>::new();
    let mut queue = Vec::new();
    let mut idx_to_path = FxHashMap::default();
    let mut path_to_idx = FxHashMap::default();
    let mut command;
    let mut node_idx = graph.add_node(0);
    let mut prev_path;
    let mut curr_path;
    let mut file_size;

    queue.push("/".to_string());
    idx_to_path.insert(node_idx, "/".to_string());
    path_to_idx.insert("/".to_string(), node_idx);

    // Parse input of the form
    for line in input.lines() {
        command = line.split_ascii_whitespace().collect::<Vec<_>>();
        match (command[0], command[1]) {
            ("$", "cd") => match command[2] {
                "/" => {
                    queue.clear();
                    queue.push("/".to_string());
                }
                ".." => {
                    queue.pop();
                }
                _ => {
                    prev_path = queue.join("/");
                    queue.push(command[2].to_string());

                    if path_to_idx.contains_key(&queue.join("/")) {
                        continue;
                    }

                    curr_path = queue.join("/");
                    node_idx = graph.add_node(0);
                    idx_to_path.insert(node_idx, curr_path.clone());
                    path_to_idx.insert(curr_path.clone(), node_idx);
                    graph.add_edge(*path_to_idx.get(&prev_path).unwrap(), node_idx, 0);
                }
            },
            ("$", _) => {}
            ("dir", _) => {}
            (_, _) => {
                file_size = command[0].parse::<usize>().unwrap();
                prev_path = queue.join("/");

                *graph
                    .node_weight_mut(*path_to_idx.get(&prev_path).unwrap())
                    .unwrap() += file_size;
            }
        }
    }

    // DFS
    let mut dfs;
    let mut res = 0;
    let mut sum;
    for (idx, _) in idx_to_path.iter() {
        dfs = Dfs::new(&graph, *idx);
        sum = 0;
        while let Some(nx) = dfs.next(&graph) {
            sum += graph[nx];
        }

        if sum <= 100000 {
            res += sum;
        }
    }

    println!("Res: {}", res);
}

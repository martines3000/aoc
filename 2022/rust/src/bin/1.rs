extern crate aoc2022;

use crate::aoc2022::utils::scanner::Scanner;

fn main() {
    println!("Hello, world!");

    // Scanner
    let mut scanner = Scanner::new();
    println!(
        "{} {} {} {:?} {:?}",
        scanner.i64(),
        scanner.i32(),
        scanner.usize(),
        scanner.f64(),
        scanner.f32()
    );
}

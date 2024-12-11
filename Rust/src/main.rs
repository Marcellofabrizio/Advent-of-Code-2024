use std::{fs, result, fmt::format};
mod day10;

fn main() {
    println!("Solving AOC 2024\n");
    let input = fs::read_to_string("day10/data").expect("File not found");
    day10::solve1(&input);
    day10::solve2(&input);
}

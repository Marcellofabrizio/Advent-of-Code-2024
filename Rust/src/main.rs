use std::{fs, result, fmt::format};
mod day1;

fn main() {
    println!("Solving AOC 2024\n");
    let input = fs::read_to_string("day1/data2").expect("File not found");
    // day1::solve1(&input);
    day1::solve2(&input);
}

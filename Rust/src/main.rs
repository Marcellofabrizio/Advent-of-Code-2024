use std::{fs, result, fmt::format};
mod day4;

fn main() {
    println!("Solving AOC 2024\n");
    let input = fs::read_to_string("day4/data").expect("File not found");
    day4::solve1(&input);
    day4::solve2(&input);
}

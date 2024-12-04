use std::{fs, result, fmt::format};
mod day3;

fn main() {
    println!("Solving AOC 2024\n");
    let input = fs::read_to_string("day3/data").expect("File not found");
    day3::solve1_lexer(&input);
    // day3::solve2(&input);
}

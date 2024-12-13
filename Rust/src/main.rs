use std::fs;
mod day6;

fn main() {
    println!("Solving AOC 2024\n");
    let input = fs::read_to_string("day6/data").expect("File not found");
    day6::solve1(&input);
    day6::solve2(&input);
}

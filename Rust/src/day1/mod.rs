use std::fs;

pub fn solve1(input: &String) {
    let mut first_col: Vec<i32> = Vec::new();
    let mut second_col: Vec<i32> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    for line in &lines {
        let split_line: Vec<&str> = line.split_whitespace().collect(); 
        if let (Some(first), Some(second)) = (split_line.get(0), split_line.get(1)) {
            first_col.push(first.parse().unwrap_or_else(|_| panic!("Invalid integer: {}", first)));
            second_col.push(second.parse().unwrap_or_else(|_| panic!("Invalid integer: {}", second)));
        } else {
            panic!("Line does not have two columns: {}", line);
        }
    }

    first_col.sort();
    second_col.sort();


    let sum: i32 = first_col
        .iter()
        .zip(&second_col)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{}", sum);
}

pub fn solve2(input: &String) {
    let mut first_col: Vec<i32> = Vec::new();
    let mut second_col: Vec<i32> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if let (Some(first), Some(second)) = (split_line.get(0), split_line.get(1)) {
            first_col.push(first.parse().unwrap_or_else(|_| panic!("Invalid integer: {}", first)));
            second_col.push(second.parse().unwrap_or_else(|_| panic!("Invalid integer: {}", second)));
        } else {
            panic!("Line does not have two columns: {}", line);
        }
    }

    first_col.sort();
    second_col.sort();


    let sum: i32 = first_col
        .iter()
        .map(|&x| {
            second_col
                .iter()
                .filter(|&&y| y == x)
                .count() as i32 * x
        })
        .sum();

    println!("{}", sum)
}


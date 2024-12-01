use std::fs;

pub fn solve1(input: &String) {
    let mut first_col: Vec<i32> = [].to_vec();
    let mut second_col: Vec<i32> = [].to_vec();

    let lines: Vec<&str> = input.lines().collect();
    for line in &lines {
        let split_line: Vec<&str> = line.split("   ").collect(); 
        first_col.push(split_line[0].parse::<i32>().unwrap());
        second_col.push(split_line[1].parse::<i32>().unwrap());
    }

    first_col.sort();
    second_col.sort();


    let mut sum: i32 = 0;

    for (i, _) in first_col.iter().enumerate() {
        let diff = first_col[i] - second_col[i];
        sum += diff.abs();
    }

    println!("{}", sum)
}

pub fn solve2(input: &String) {
    let mut first_col: Vec<i32> = [].to_vec();
    let mut second_col: Vec<i32> = [].to_vec();

    let lines: Vec<&str> = input.lines().collect();
    for line in &lines {
        let split_line: Vec<&str> = line.split("   ").collect(); 
        first_col.push(split_line[0].parse::<i32>().unwrap());
        second_col.push(split_line[1].parse::<i32>().unwrap());
    }

    first_col.sort();
    second_col.sort();


    let mut sum: i32 = 0;

    for (i, _) in first_col.iter().enumerate() {
        let count = second_col.iter().filter(|&n| *n == first_col[i]).count();
        sum += first_col[i] * count as i32;
    }

    println!("{}", sum)
}


pub fn solve1(input: &String) {
    /*
    S * * S * * S
    * A * A * A *
    * * M M M * *
    S A M X M A S
    * * M M M * *
    * A * A * A *
    S * * S * * S
    */

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 'X' {
                sum += check_xmas(&matrix, row, col);
            }
        }
    }

    println!("{}", sum);
}

fn check_xmas(matrix: &[Vec<char>], row: usize, col: usize) -> i32 {
    const DIRECTIONS: [(isize, isize); 8] = [
        (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1),
    ];

    let mut occurrence = 0;

    for &(dr, dc) in &DIRECTIONS {
        let mut word = String::new();
        for i in 0..4 {
            let r = row as isize + dr * i;
            let c = col as isize + dc * i;

            if let Some(row) = matrix.get(r as usize) {
                if let Some(&ch) = row.get(c as usize) {
                    word.push(ch);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if word == "XMAS" {
            occurrence += 1;
        }
    }

    occurrence
}

pub fn solve2(input: &str) {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 'A' {
                sum += check_x_mas(&matrix, row, col);
            }
        }
    }

    println!("{}", sum);
}

fn check_x_mas(matrix: &[Vec<char>], row: usize, col: usize) -> i32 {
    const DIRECTIONS: [[(isize, isize); 3]; 2] = [
        [(-1, -1), (0, 0), (1, 1)],
        [(1, -1), (0, 0), (-1, 1)],
    ];

    let mut mas_occurrence = 0;

    for &pattern in &DIRECTIONS {
        let mut word = String::new();

        for &(dr, dc) in &pattern {
            let r = row as isize + dr;
            let c = col as isize + dc;

            if let Some(row) = matrix.get(r as usize) {
                if let Some(&ch) = row.get(c as usize) {
                    word.push(ch);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if word == "MAS" || word == "SAM" {
            mas_occurrence += 1;
        }
    }

    (mas_occurrence == 2) as i32
}
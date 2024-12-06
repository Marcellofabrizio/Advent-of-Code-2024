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

    let kernel: Vec<Vec<char>> = vec![
        vec!['S', '*', '*', 'S', '*', '*', 'S'],
        vec!['*', 'A', '*', 'A', '*', 'A', '*'],
        vec!['*', '*', 'M', 'M', 'M', '*', '*'],
        vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'],
        vec!['*', '*', 'M', 'M', 'M', '*', '*'],
        vec!['*', 'A', '*', 'A', '*', 'A', '*'],
        vec!['S', '*', '*', 'S', '*', '*', 'S'],
    ];

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut sum = 0;

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 'X' {
                // for krow in 0..7 {
                //     for kcol in 0..7 {
                //         let matrix_row = row as isize + krow as isize - 3 as isize;
                //         let matrix_col = col as isize + kcol as isize - 3 as isize;

                //         if matrix_row < 0
                //             || matrix_row >= rows as isize
                //             || matrix_col < 0
                //             || matrix_col >= cols as isize
                //         {
                //             continue;
                //         }

                //         println!("{} {}: {}", krow, kcol, kernel[krow][kcol]);
                //         println!(
                //             "{} {}: {}",
                //             matrix_row,
                //             matrix_col,
                //             matrix[matrix_row as usize][matrix_col as usize]
                //         )
                //     }
                // }

                sum += check_xmas(&matrix, row, col);
            }
        }
    }

    println!("{}", sum)
}

fn check_xmas(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let mut occurence = 0;

    for &(dr, dc) in &directions {
        let mut word = String::new();
        for i in 0..4 {
            let r = row as isize + dr * i;
            let c = col as isize + dc * i;
            if r < 0 || c < 0 || r >= matrix.len() as isize || c >= matrix[0].len() as isize {
                break;
            }
            word.push(matrix[r as usize][c as usize]);
        }
        if word == "XMAS" {
            occurence += 1;
        }
    }
    occurence
}

pub fn solve2(input: &String) {
    /*
    M * S
    * A *
    S * M
    */

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut sum = 0;

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 'A' {
                sum += check_x_mas(&matrix, row, col);
            }
        }
    }

    println!("{}", sum)
}

fn check_x_mas(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let directions = [[(-1, -1), (0, 0), (1, 1)], [(1, -1), (0, 0), (-1, 1)]];

    let mut mas_occurence = 0;
    let mut occurence = 0;
    let allowed: Vec<char> = vec!['M', 'A', 'S'];

    for &dg in &directions {
        let mut word = String::new();
        for &(dr, dc) in &dg {
            // for i in (0..3).rev() {
            let r = row as isize + dr;
            let c = col as isize + dc;
            if r < 0 || c < 0 || r >= matrix.len() as isize || c >= matrix[0].len() as isize {
                break;
            }

            word.push(matrix[r as usize][c as usize]);

            // if allowed.contains(&matrix[r as usize][c as usize]) {
            //     println!("{} at {} {}", word, r, c);
            // }

            // }
        }

        if word == "MAS" || word == "SAM" {
            mas_occurence += 1;
        }
    }

    if mas_occurence == 2 {
        return 1;
    }

    0
}
